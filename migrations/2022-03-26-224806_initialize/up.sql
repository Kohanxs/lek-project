-- Your SQL goes here
create table users
(
    id            serial
        constraint users_pk
            primary key,
    username      varchar(32) not null,
    password_hash varchar(64) not null,
    salt          varchar(22) not null,
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
    correct_answer integer,
    category_fk    integer
        constraint questions_category_id_fk
            references category
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
            references questions,
    likes            integer default 1 not null
);

create table comment_likes
(
    user_fk    int
        constraint comment_likes_users_id_fk
            references users
            on delete cascade,
    comment_fk int
        constraint comment_likes_comments_id_fk
            references comments
            on delete cascade,
    constraint comment_likes_pk
        primary key (comment_fk, user_fk)
);