use std::cmp::Ordering;
use std::mem::size_of;
use std::collections::HashMap;
use crate::enums::BinaryTree::NonEmpty;
use crate::enums::BinaryTree::Empty;

pub fn enums_work(){
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);

    // Casting
    assert_eq!(HttpStatus::Ok as i32, 200);

    let four_score_and_seven_years_ago =
        RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);

    let three_hours_from_now =
        RoughTime::InTheFuture(TimeUnit::Hours, 3);

    let unit_sphere = Shape::Sphere {
        center: 23,
        radius: 1.0,
    };


    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: Empty,
    }));

    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: Empty,
    }));

}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years,
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }

    
}
// Tuple variants.
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

// Struct variants.
enum Shape {
    Sphere { center: i32, radius: f32 },
    Cuboid { corner1: i32, corner2: i32 },
}

// All variants in on enum
enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: String,
        cdr: String,
    },
}


enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}



