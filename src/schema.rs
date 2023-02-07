table! {
    category (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    comment_user (comment_fk, user_fk) {
        comment_fk -> Int4,
        user_fk -> Int4,
    }
}

table! {
    question_category (question_fk, category_fk) {
        question_fk -> Int4,
        category_fk -> Int4,
    }
}

table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        suggested_answer -> Nullable<Int4>,
        users_fk -> Int4,
        questions_fk -> Int4,
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
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        nickname -> Varchar,
        is_admin -> Bool,
    }
}

joinable!(comments -> questions (questions_fk));
joinable!(comments -> users (users_fk));
joinable!(question_category -> questions (question_fk));
joinable!(question_category -> category (category_fk));
joinable!(comment_user -> comments (comment_fk));
joinable!(comment_user -> users (user_fk));

allow_tables_to_appear_in_same_query!(
    category,
    comments,
    questions,
    users,
    comment_user,
    question_category,
);
