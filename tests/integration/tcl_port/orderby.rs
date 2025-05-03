#[cfg(test)]
mod tests {
    use crate::db_test;

    db_test!(
        basic_order_by_1,
        "SELECT * FROM products ORDER BY price",
        [
            [9, "boots", 1.0],
            [3, "shirt", 18.0],
            [4, "sweater", 25.0],
            [10, "coat", 33.0],
            [6, "shorts", 70.0],
            [5, "sweatshirt", 74.0],
            [7, "jeans", 78.0],
            [1, "hat", 79.0],
            [11, "accessories", 81.0],
            [2, "cap", 82.0],
            [8, "sneakers", 82.0]
        ]
    );

    db_test!(
        basic_order_by_2,
        "SELECT * FROM products ORDER BY price DESC",
        [
            [2, "cap", 82.0],
            [8, "sneakers", 82.0],
            [11, "accessories", 81.0],
            [1, "hat", 79.0],
            [7, "jeans", 78.0],
            [5, "sweatshirt", 74.0],
            [6, "shorts", 70.0],
            [10, "coat", 33.0],
            [4, "sweater", 25.0],
            [3, "shirt", 18.0],
            [9, "boots", 1.0]
        ]
    );

    db_test!(
        basic_order_by_and_limit_1,
        "SELECT * FROM products ORDER BY name LIMIT 5",
        [
            [11, "accessories", 81.0],
            [9, "boots", 1.0],
            [2, "cap", 82.0],
            [10, "coat", 33.0],
            [1, "hat", 79.0]
        ]
    );

    db_test!(
        basic_order_by_and_limit_2,
        "SELECT * FROM products ORDER BY name DESC LIMIT 5",
        [
            [5, "sweatshirt", 74.0],
            [4, "sweater", 25.0],
            [8, "sneakers", 82.0],
            [6, "shorts", 70.0],
            [3, "shirt", 18.0]
        ]
    );

    db_test!(
        basic_order_by_and_limit_2_1,
        "SELECT id, name FROM products ORDER BY name LIMIT 5",
        [
            [11, "accessories"],
            [9, "boots"],
            [2, "cap"],
            [10, "coat"],
            [1, "hat"]
        ]
    );

    db_test!(
        basic_order_by_and_limit_2_2,
        "SELECT id, name FROM products ORDER BY name DESC LIMIT 5",
        [
            [5, "sweatshirt"],
            [4, "sweater"],
            [8, "sneakers"],
            [6, "shorts"],
            [3, "shirt"]
        ]
    );

    db_test!(
        basic_order_by_and_limit_3_1,
        "SELECT price, name FROM products WHERE price > 70 ORDER BY name",
        [
            [81.0, "accessories"],
            [82.0, "cap"],
            [79.0, "hat"],
            [78.0, "jeans"],
            [82.0, "sneakers"],
            [74.0, "sweatshirt"]
        ]
    );

    db_test!(
        basic_order_by_and_limit_3_2,
        "SELECT price, name FROM products WHERE price > 70 ORDER BY name DESC",
        [
            [74.0, "sweatshirt"],
            [82.0, "sneakers"],
            [78.0, "jeans"],
            [79.0, "hat"],
            [82.0, "cap"],
            [81.0, "accessories"]
        ]
    );

    db_test!(
        order_by_qualified_1,
        "SELECT u.first_name FROM users u ORDER BY u.first_name LIMIT 1",
        ["Aaron"]
    );

    db_test!(
        order_by_qualified_2,
        "SELECT u.first_name FROM users u ORDER BY u.first_name DESC LIMIT 1",
        ["Zoe"]
    );

    db_test!(
        order_by_column_number_1,
        "SELECT first_name, last_name, age FROM users ORDER BY 3, 2 LIMIT 2",
        [["Teresa", "Allen", 1], ["David", "Baker", 1]]
    );

    db_test!(
        order_by_column_number_2,
        "SELECT first_name, last_name, age FROM users ORDER BY 3 DESC, 2 ASC LIMIT 2",
        [["Connor", "Adkins", 100], ["John", "Bell", 100]]
    );

    db_test!(
        order_by_column_number_3,
        "SELECT first_name, last_name, age FROM users ORDER BY 3 ASC, 2 DESC LIMIT 2",
        [["Kyle", "Wolf", 1], ["Jason", "Williams", 1]]
    );

    db_test!(
        order_by_column_number_4,
        "SELECT first_name, last_name, age FROM users ORDER BY 3 ASC, 2 DESC LIMIT 10",
        [
            ["Kyle", "Wolf", 1],
            ["Jason", "Williams", 1],
            ["Tracey", "Williams", 1],
            ["Jessica", "Werner", 1],
            ["Jasmine", "Warren", 1],
            ["Dennis", "Ward", 1],
            ["Whitney", "Walker", 1],
            ["Robert", "Villanueva", 1],
            ["Cynthia", "Thomas", 1],
            ["Brandon", "Tate", 1]
        ]
    );

    db_test!(
        order_by_case_insensitive_aggregate,
        "SELECT u.first_name, sum(u.age) FROM users u GROUP BY u.first_name ORDER BY SUM(u.aGe) DESC LIMIT 10",
        [
            ["Michael", 11204],
            ["David", 8758],
            ["Robert", 8109],
            ["Jennifer", 7700],
            ["John", 7299],
            ["Christopher", 6397],
            ["James", 5921],
            ["Joseph", 5711],
            ["Brian", 5059],
            ["William", 5047]
        ]
    );

    db_test!(
        order_by_agg_not_mentioned_in_select,
        "SELECT u.first_name, length(group_concat(u.last_name)) FROM users u GROUP BY u.first_name ORDER BY max(u.email) DESC LIMIT 5",
        [
            ["Louis", 65],
            ["Carolyn", 118],
            ["Katelyn", 40],
            ["Erik", 88],
            ["Collin", 15]
        ]
    );

    db_test!(
        case_insensitive_alias,
        "SELECT u.first_name AS fF, count(1) > 0 AS cC FROM users u WHERE fF = 'Jamie' GROUP BY fF ORDER BY cC",
        [["Jamie", 1]]
    );

    db_test!(
        age_idx_order_desc,
        "SELECT first_name FROM users ORDER BY age DESC LIMIT 3",
        [["Robert"], ["Sydney"], ["Matthew"]]
    );

    db_test!(
        rowid_or_integer_pk_desc,
        "SELECT first_name FROM users ORDER BY id DESC LIMIT 3",
        [["Nicole"], ["Gina"], ["Dorothy"]]
    );

    // These two following tests may seem dumb but they verify that index scanning by age_idx doesn't drop any rows due to BTree bugs
    db_test!(
        orderby_asc_verify_rows,
        "SELECT count(1) FROM (SELECT * FROM users ORDER BY age DESC)",
        [10000]
    );

    // These two following tests may seem dumb but they verify that index scanning by age_idx doesn't drop any rows due to BTree bugs
    db_test!(
        orderby_desc_verify_rows,
        "SELECT count(1) FROM (SELECT * FROM users ORDER BY age DESC)",
        [10000]
    );

    db_test!(
        orderby_desc_with_offset,
        "SELECT first_name, age FROM users ORDER BY age DESC LIMIT 3 OFFSET 666",
        [["Francis", 94], ["Matthew", 94], ["Theresa", 94]]
    );

    db_test!(
        orderby_desc_with_filter,
        "SELECT first_name, age FROM users WHERE age <= 50 ORDER BY age DESC LIMIT 5",
        [
            ["Gerald", 50],
            ["Nicole", 50],
            ["Tammy", 50],
            ["Marissa", 50],
            ["Daniel", 50]
        ]
    );

    db_test!(
        orderby_asc_with_filter_range,
        "SELECT first_name, age FROM users WHERE age <= 50 AND age >= 49 ORDER BY age ASC LIMIT 5",
        [
            ["William", 49],
            ["Jennifer", 49],
            ["Robert", 49],
            ["David", 49],
            ["Stephanie", 49]
        ]
    );

    db_test!(
        orderby_desc_with_filter_id_lt,
        "SELECT id FROM users WHERE id < 6666 ORDER BY id DESC LIMIT 5",
        [[6665], [6664], [6663], [6662], [6661]]
    );

    db_test!(
        orderby_desc_with_filter_id_le,
        "SELECT id FROM users WHERE id <= 6666 ORDER BY id DESC LIMIT 5",
        [[6666], [6665], [6664], [6663], [6662]]
    );
}
