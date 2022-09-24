use std::{sync::Arc};

use crate::digit::Digit;

#[derive(Debug, PartialEq)]
pub enum FieldGroupType {
    Row(Digit),
    Column(Digit),
    Shape(Digit)
}

pub struct FieldGroup {
    fields: Vec<Arc<Field>>,
    group_type: FieldGroupType
}

pub struct Field {
    status: Option<Digit>
}

pub struct Game {
    grid: [[Arc<Field>; 9]; 9],
    groups: Vec<FieldGroup>
}

impl Game {
    pub fn new(definition: [[Option<Digit>; 9]; 9]) -> Game {
        let fields = definition.map(|row| 
            row.map(|field| Arc::new(Field { status: field}))
        );
        let rows: Vec<FieldGroup> =(0..9).map(|x| 
            FieldGroup { 
                group_type: FieldGroupType::Row(Digit::try_from(x+1).unwrap()),
                fields: fields[usize::from(x)].to_vec()
            }).collect();
        let columns: Vec<FieldGroup> =(0..9).map(|x| 
            FieldGroup { 
                group_type: FieldGroupType::Column(Digit::try_from(x+1).unwrap()),
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
                group_type: FieldGroupType::Shape(Digit::try_from(x+1).unwrap()),
                fields: fields
            }
        }).collect();
        Game {
            grid: fields,
            groups: rows.into_iter().chain(columns.into_iter().chain(shapes.into_iter())).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::*;
    
    #[test]
    fn game_creation_works() {
        let game = Game::new([
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
        assert_eq!(game.groups.get(0).unwrap().group_type,FieldGroupType::Row(Digit::new(1).unwrap()));
        assert_eq!(game.groups.get(18).unwrap().group_type,FieldGroupType::Shape(Digit::new(1).unwrap()));
    }
}