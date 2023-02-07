-- Your SQL goes here
create table users
(
    id            serial
        constraint users_pk
            primary key,
    username      varchar(32) not null,
    password_hash varchar(64) not null,
    nickname      varchar(32) not null
);

create unique index users_nickname_uindex
    on users (nickname);

create unique index users_username_uindex
    on users (username);

create table category
(
    id   serial
        constraint category_pk
            primary key,
    name varchar(32) not null
);

create table questions
(
    id             serial
        constraint questions_pk
            primary key,
    content        text not null,
    answer_1       text not null,
    answer_2       text not null,
    answer_3       text not null,
    answer_4       text not null,
    answer_5       text not null,
    correct_answer integer
);

create table comments
(
    id               serial
        constraint comments_pk
            primary key,
    content          text not null,
    suggested_answer integer,
    users_fk         integer not null
        constraint comments_users_id_fk
            references users,
    questions_fk     integer not null
        constraint comments_questions_id_fk
            references questions
);

create table question_category
(
    question_fk    int
        constraint question_category_question_id_fk
            references questions
            on delete cascade,
    category_fk int
        constraint question_category_category_id_fk
            references category
            on delete cascade,
    constraint question_category_pk
        primary key (question_fk, category_fk)
)

create table comment_user
(
    user_fk    int
        constraint comment_user_users_id_fk
            references users
            on delete cascade,
    comment_fk int
        constraint comment_user_comments_id_fk
            references comments
            on delete cascade,
    constraint comment_user_pk
        primary key (comment_fk, user_fk)
);