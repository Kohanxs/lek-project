table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        suggested_answer -> Nullable<Int4>,
        users_fk -> Int4,
        questions_fk -> Int4,
        likes -> Int4,
    }
}

table! {
    questions (id) {
        id -> Int4,
        content -> Text,
        answer_1 -> Text,
        answer_2 -> Text,
        answer_3 -> Text,
        answer_4 -> Text,
        answer_5 -> Text,
        correct_answer -> Nullable<Int4>,
        category_fk -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        nickname -> Varchar,
    }
}

joinable!(comments -> questions (questions_fk));
joinable!(comments -> users (users_fk));
joinable!(questions -> category (category_fk));

allow_tables_to_appear_in_same_query!(
    category,
    comments,
    questions,
    users,
);
