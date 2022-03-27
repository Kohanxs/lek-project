table! {
    questions (id) {
        id -> Integer,
        text -> Text,
        answer_1 -> Text,
        answer_2 -> Text,
        anwser_3 -> Text,
        anwser_4 -> Text,
        answer_5 -> Text,
        correct_answer -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        password_hash -> Integer,
        user_name -> Text,
        salt -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    questions,
    users,
);
