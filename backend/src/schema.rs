// @generated automatically by Diesel CLI.

diesel::table! {
    contest (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        uid -> Int4,
    }
}

diesel::table! {
    participates (uid, cid) {
        uid -> Int4,
        cid -> Int4,
        score -> Int4,
        official -> Bool,
    }
}

diesel::table! {
    problems (id) {
        id -> Int4,
        name -> Varchar,
        author -> Varchar,
        contest -> Varchar,
        statement -> Varchar,
        rating -> Int4,
        uid -> Int4,
    }
}

diesel::table! {
    submissions (id) {
        id -> Int4,
        user_id -> Int4,
        problem_id -> Int4,
        source_code -> Varchar,
        score -> Int4,
        language -> Varchar,
    }
}

diesel::table! {
    usercredentials (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        school -> Varchar,
        bio -> Varchar,
        teacher -> Varchar,
    }
}

diesel::joinable!(contest -> users (uid));
diesel::joinable!(participates -> contest (cid));
diesel::joinable!(participates -> users (uid));
diesel::joinable!(problems -> users (uid));
diesel::joinable!(submissions -> problems (problem_id));
diesel::joinable!(submissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contest,
    participates,
    problems,
    submissions,
    usercredentials,
    users,
);
