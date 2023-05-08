// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        user_id -> Int4,
        note -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    likes (post_id, user_id) {
        post_id -> Int4,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        heading -> Nullable<Varchar>,
        sub_heading -> Nullable<Varchar>,
        caption -> Nullable<Varchar>,
        cooking_duration -> Nullable<Time>,
        tags -> Nullable<Array<Nullable<Text>>>,
        visuals -> Nullable<Array<Nullable<Text>>>,
        ingredients -> Nullable<Array<Nullable<Text>>>,
        steps -> Nullable<Array<Nullable<Text>>>,
        likes_count -> Nullable<Int4>,
        comments_count -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    test (id) {
        id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        email -> Varchar,
        avatar -> Nullable<Varchar>,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(likes -> posts (post_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    likes,
    posts,
    test,
    users,
);
