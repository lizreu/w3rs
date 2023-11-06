use std::{
    collections::{BTreeMap, HashMap},
    iter::Peekable,
    str,
};

use atoi::atoi;
use num_traits::Num;

use crate::parser::crlf::CrlfLines;

fn strip_field(src: &[u8]) -> (&[u8], &[u8]) {
    let mut i = 0;
    while i < src.len() && src[i] != b';' {
        i += 1;
    }

    if i == src.len() {
        (src, &[])
    } else {
        (&src[0..i], &src[i + 1..src.len()])
    }
}

#[derive(Debug)]
pub enum Value<'src> {
    Str(&'src str),
    Num(&'src str),
    Empty,
}

impl<'src> Value<'src> {
    fn from_slice(src: &'src [u8]) -> Value<'src> {
        if src.is_empty() {
            Value::Empty
        } else if src[0] == b'"' {
            Value::Str(Self::parse_string(src))
        } else {
            Value::Num(str::from_utf8(src).unwrap())
        }
    }

    fn parse_string(src: &'src [u8]) -> &'src str {
        let mut i = 1;

        while i < src.len() && src[i] != b'"' {
            i += 1;
        }

        str::from_utf8(&src[1..i]).unwrap()
    }

    pub fn as_raw(&self) -> Option<&'src str> {
        match self {
            Value::Str(value) => Some(value),
            Value::Num(value) => Some(value),
            Value::Empty => None,
        }
    }

    pub fn as_str(&self) -> Option<&'src str> {
        match self {
            Value::Str(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_num<N: Num>(&self) -> Option<N> {
        match self {
            Value::Num(value) => Some(N::from_str_radix(value, 10).ok().unwrap()),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Field<'src> {
    Col(u32),
    Row(u32),
    Value(&'src [u8]),
    Unknown,
}

impl<'src> Field<'src> {
    fn from_slice(src: &'src [u8]) -> (Option<Field<'src>>, &'src [u8]) {
        let (field, rest) = strip_field(src);

        if field.is_empty() {
            return (None, rest);
        }

        let field = match field[0] {
            b'X' => atoi(&field[1..]).map(Field::Col),
            b'Y' => atoi(&field[1..]).map(Field::Row),
            b'K' => Some(Field::Value(&field[1..])),
            _ => Some(Field::Unknown),
        };

        (field, rest)
    }
}

#[derive(Debug)]
pub struct Cell<'src> {
    column: u32,
    row:    Option<u32>,
    value:  Value<'src>,
}

impl<'src> Cell<'src> {
    fn empty(column: u32) -> Cell<'src> {
        Cell {
            column,
            row: None,
            value: Value::Empty,
        }
    }

    fn from_bytes(src: &'src [u8]) -> Option<Cell<'src>> {
        let (field, mut rest) = strip_field(src);

        let mut cell = if field == b"C" {
            Cell {
                column: 0,
                row:    None,
                value:  Value::Empty,
            }
        } else {
            return None;
        };

        while !rest.is_empty() {
            let (field, new_rest) = Field::from_slice(rest);
            rest = new_rest;

            if let Some(field) = field {
                cell.apply_field(field);
            }
        }

        Some(cell)
    }

    fn apply_field(&mut self, field: Field<'src>) {
        match field {
            Field::Col(field_col) => self.column = field_col,
            Field::Row(field_row) => self.row = Some(field_row),
            Field::Value(field_value) => self.value = Value::from_slice(field_value),
            _ => {}
        }
    }

    pub fn value(&self) -> &Value<'src> {
        &self.value
    }

    pub fn column(&self) -> u32 {
        self.column
    }

    pub fn as_str(&self) -> Option<&'src str> {
        self.value.as_str()
    }

    pub fn as_num<N: Num>(&self) -> Option<N> {
        self.value.as_num()
    }
}

struct Cells<'src> {
    lines: CrlfLines<'src>,
}

impl<'src> Iterator for Cells<'src> {
    type Item = Cell<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        for line in self.lines.by_ref() {
            if let Some(cell) = Cell::from_bytes(line) {
                return Some(cell);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct RawRow<'src> {
    pub position: u32,
    pub cells:    Vec<Cell<'src>>,
}

pub struct RawRows<'src> {
    cells: Peekable<Cells<'src>>,
}

impl<'src> RawRows<'src> {
    pub fn new(source: &'src [u8]) -> Self {
        Self {
            cells: Cells {
                lines: CrlfLines::new(source),
            }
            .peekable(),
        }
    }
}

impl<'src> Iterator for RawRows<'src> {
    type Item = RawRow<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cells.peek().is_none() {
            None
        } else {
            self.cells.find(|r| r.row.is_some()).map(|cell| {
                let mut cells = Vec::new();
                let position = cell.row.unwrap();
                cells.push(cell);

                while let Some(peeked) = self.cells.peek() {
                    while peeked.column > (cells.len() + 1) as u32 {
                        cells.push(Cell::empty((cells.len() + 1) as u32));
                    }

                    if peeked.row.is_some() {
                        break;
                    } else {
                        cells.push(self.cells.next().unwrap())
                    }
                }

                RawRow { position, cells }
            })
        }
    }
}

#[derive(Clone)]
pub struct ColumnNames<'src> {
    columns: Vec<&'src str>,
    by_name: HashMap<&'src str, usize, ahash::RandomState>,
}

impl<'src> ColumnNames<'src> {
    fn new(row: RawRow<'src>) -> ColumnNames<'src> {
        let mut columns = Vec::new();
        let mut by_name = HashMap::with_hasher(ahash::RandomState::new());

        for cell in row.cells {
            let name = cell.value.as_str().unwrap_or("");

            columns.push(name);
            by_name.insert(name, columns.len() - 1);
        }

        ColumnNames { columns, by_name }
    }

    pub fn idx_by_name(&self, name: &str) -> Option<u32> {
        self.by_name.get(name).map(|idx| *idx as u32)
    }

    pub fn name_by_idx(&self, idx: u32) -> Option<&'src str> {
        self.columns.get(idx as usize).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &'src str> + '_ {
        self.columns.iter().copied()
    }
}

/// A single row of an SLK table.
pub struct TableRow<'a, 'src: 'a> {
    rows: &'a TableRows<'src>,
    row:  RawRow<'src>,
}

impl<'a, 'src: 'a> TableRow<'a, 'src> {
    pub fn position(&self) -> u32 {
        self.row.position
    }

    pub fn by_column<'b>(&'b self, column: &str) -> Option<&'b Cell> {
        let columns = &self.rows.columns;
        let idx = columns.idx_by_name(column)?;
        self.row.cells.get(idx as usize)
    }

    pub fn by_idx(&self, column: u32) -> Option<&Cell> {
        self.row.cells.get(column as usize)
    }

    pub fn column_name(&self, column: u32) -> Option<&'src str> {
        self.rows.columns.name_by_idx(column)
    }

    pub fn cells_with_names<'b>(&'b self) -> impl Iterator<Item = (&'src str, &'b Cell)> + 'b {
        self.rows.columns.iter().zip(self.row.cells.iter())
    }
}

/// A parser for the rows of an SLK table.
pub struct TableRows<'src> {
    rows:    RawRows<'src>,
    columns: ColumnNames<'src>,
}

impl<'src> TableRows<'src> {
    pub fn new(source: &'src [u8]) -> Option<Self> {
        let mut rows = RawRows::new(source);
        let legend = ColumnNames::new(rows.next()?);

        Some(TableRows {
            rows,
            columns: legend,
        })
    }

    pub fn advance<'a>(&'a mut self) -> Option<TableRow<'a, 'src>> {
        let raw_row = self.rows.next();
        raw_row.map(|row| TableRow { rows: self, row })
    }

    pub fn has_next(&mut self) -> bool {
        self.rows.cells.peek().is_some()
    }
}
