table! {
    category (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    comments (id) {
        id -> Integer,
        content -> Nullable<Text>,
        answer -> Nullable<Integer>,
        users_fk -> Integer,
        questions_fk -> Integer,
    }
}

table! {
    questions (id) {
        id -> Integer,
        content -> Text,
        answer_1 -> Text,
        answer_2 -> Text,
        answer_3 -> Text,
        answer_4 -> Text,
        answer_5 -> Text,
        correct_answer -> Nullable<Integer>,
        category_fk -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        password_hash -> Text,
        salt -> Text,
        nickname -> Text,
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
