use std::{sync::Arc};

use crate::digit::Digit;

#[derive(Debug, PartialEq)]
pub enum FieldGroupType {
    Row(u8),
    Column(u8),
    Shape(u8)
}

pub struct FieldGroup {
    fields: Vec<Field>,
    group_type: FieldGroupType
}

#[derive(Clone, Copy)]
pub struct Field {
    content: Option<Digit>
}

#[derive(Clone, Copy)]
pub struct DigitPosition {
    content: Digit,
    pos_x: u8,
    pos_y: u8
}

pub struct Game {
    grid: [[Field; 9]; 9],
    groups: Vec<FieldGroup>
}

impl From<[[Option<Digit>; 9]; 9]> for Game {
    fn from(definition: [[Option<Digit>; 9]; 9]) -> Self {
        let fields = definition.map(|row| 
            row.map(|field| Field { content: field})
        );
        let rows: Vec<FieldGroup> =(0..9).map(|x| 
            FieldGroup { 
                group_type: FieldGroupType::Row(x),
                fields: fields[usize::from(x)].to_vec()
            }).collect();
        let columns: Vec<FieldGroup> =(0..9).map(|x| 
            FieldGroup { 
                group_type: FieldGroupType::Column(x),
                fields: fields[..][usize::from(x)].to_vec()
            }).collect();
        let shapes: Vec<FieldGroup> =(0..9).map(|x| {
            let row_lower_bound = usize::from((x / 3) * 3);
            let row_upper_bound = row_lower_bound + 3;
            let column_lower_bound = usize::from((x % 3) *3);
            let column_upper_bound = column_lower_bound + 3;
            let rows =  fields[row_lower_bound..row_upper_bound].to_vec();
            let fields = rows.into_iter().flat_map(|row| row[column_lower_bound..column_upper_bound].to_vec()).collect();
            FieldGroup { 
                group_type: FieldGroupType::Shape(x),
                fields: fields
            }
        }).collect();
        Game {
            grid: fields,
            groups: rows.into_iter().chain(columns.into_iter().chain(shapes.into_iter())).collect()
        }
    }
}

impl TryFrom<Vec<DigitPosition>> for Game {
    type Error = &'static str;

    fn try_from(positions: Vec<DigitPosition>) -> Result<Self, Self::Error> {
        if !positions.clone().into_iter().all(|f| f.pos_x < 9 && f.pos_y < 9) {
            Err("Positions not valid - must be below 9")
        } else {
            let mut arr: [[Option<Digit>; 9]; 9] = Default::default();
            for pos in positions.into_iter() {
                arr[usize::from(pos.pos_x)][usize::from(pos.pos_y)] = Some(pos.content)
            }
            Ok(Game::from(arr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn game_creation_with_array_works() {
        let game = Game::from([
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None],
            [Digit::new(1), Digit::new(2), Digit::new(3), None, None, None, None, None, None]
        ]);
        assert_eq!(game.groups.get(0).unwrap().group_type,FieldGroupType::Row(0));
        assert_eq!(game.groups.get(18).unwrap().group_type,FieldGroupType::Shape(0));
    }

    #[test]
    fn game_creation_with_positions() {
        let game = Game::try_from(vec![
            DigitPosition {
                content: Digit::new(1).unwrap(),
                pos_x: 0,
                pos_y: 0
            }
        ]);
        assert_eq!(game.as_ref().unwrap().groups.get(0).unwrap().group_type,FieldGroupType::Row(0));
        assert_eq!(game.as_ref().unwrap().groups.get(18).unwrap().group_type,FieldGroupType::Shape(0));
    }
}