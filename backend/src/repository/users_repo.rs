use crate::model::user::{User};
use crate::repository::{DbConn, DbError};
use crate::model::participates::Participates;
use crate::model::submission::Submission;
use crate::utils::mock::Mockable;

pub fn get_all_users(db: &mut Mockable<DbConn>) -> Result<Vec<User>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_users(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_user_by_id(db: &mut Mockable<DbConn>, uid: i32) -> Result<Option<User>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_user_by_id(inner, uid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn add_user(db: &mut Mockable<DbConn>, user: User) {
    match db {
        Mockable::Real(inner) => real::add_user(inner, user),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn delete_user(db: &mut Mockable<DbConn>, uid: i32) {
    match db {
        Mockable::Real(inner) => real::delete_user(inner, uid),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn update_user(db: &mut Mockable<DbConn>, user: User) {
    match db {
        Mockable::Real(inner) => real::update_user(inner, user),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_all_users_with_participations(db: &mut Mockable<DbConn>) -> Result<Vec<(User, Vec<Participates>)>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_users_with_participations(inner),
        Mockable::Mock => panic!("Mock not implemented!")
    }
}

pub fn get_all_users_with_submissions(db: &mut Mockable<DbConn>) -> Result<Vec<(User, Vec<Submission>)>, DbError> {
    match db {
        Mockable::Real(inner) => real::get_all_users_with_submissions(inner),
        Mockable::Mock => mock::get_all_user_with_submissions()
    }
}

mod real {
    use diesel::prelude::*;
    use crate::model::participates::Participates;
    use crate::model::submission::Submission;
    use crate::model::user::{NewUser, User};
    use crate::repository::DbError;
    use crate::schema::users::dsl::*;

    pub fn get_all_users(db: &mut PgConnection) -> Result<Vec<User>, DbError> {
        let user_list = users.load(db)?;
        Ok(user_list)
    }

    pub fn get_user_by_id(db: &mut PgConnection, uid: i32) -> Result<Option<User>, DbError> {
        let user = users.filter(id.eq(uid))
            .first::<User>(db)
            .optional()?;

        Ok(user)
    }

    pub fn add_user(db: &mut PgConnection, user: User) {
        let new_user = NewUser {
            first_name: user.first_name,
            last_name: user.last_name,
            school: user.school,
            bio: user.bio,
            teacher: user.teacher
        };
        diesel::insert_into(users).values(new_user).execute(db).unwrap();
    }

    pub fn delete_user(db: &mut PgConnection, uid: i32) {
        diesel::delete(users.filter(id.eq(uid))).execute(db).unwrap();
    }

    pub fn update_user(db: &mut PgConnection, user: User) {
        diesel::update(users.filter(id.eq(user.id))).set(user).execute(db).unwrap();
    }

    pub fn get_all_users_with_participations(db: &mut PgConnection) -> Result<Vec<(User, Vec<Participates>)>, DbError> {
        let all_users = users.load(db).unwrap();
        let participation_list = Participates::belonging_to(&all_users).load(db).unwrap();

        let participations_per_user = participation_list.grouped_by(&all_users)
            .into_iter()
            .zip(all_users)
            .map(|(participation, user)| (user, participation))
            .collect::<Vec<(User, Vec<Participates>)>>();

        Ok(participations_per_user)
    }

    pub fn get_all_users_with_submissions(db: &mut PgConnection) -> Result<Vec<(User, Vec<Submission>)>, DbError> {
        let all_users = users.load(db).unwrap();
        let submission_list = Submission::belonging_to(&all_users).load(db).unwrap();

        let submissions_per_user = submission_list.grouped_by(&all_users)
            .into_iter()
            .zip(all_users)
            .map(|(submission, user)| (user, submission))
            .collect::<Vec<(User, Vec<Submission>)>>();

        Ok(submissions_per_user)
    }
}

mod mock {
    use crate::model::submission::Submission;
    use crate::model::user::User;
    use crate::repository::DbError;

    pub fn get_all_user_with_submissions() -> Result<Vec<(User, Vec<Submission>)>, DbError> {
        Ok(
            vec![(User {
                id: 1,
                first_name: "1".to_string(),
                last_name: "1".to_string(),
                school: "1".to_string(),
                bio: "1".to_string(),
                teacher: "1".to_string(),
            }, vec![Submission {
                id: 1,
                user_id: 1,
                problem_id: 1,
                source_code: "1".to_string(),
                score: 1,
                language: "1".to_string(),
            }, Submission {
                id: 2,
                user_id: 1,
                problem_id: 2,
                source_code: "2".to_string(),
                score: 2,
                language: "2".to_string(),
            }]), (User {
                id: 2,
                first_name: "2".to_string(),
                last_name: "2".to_string(),
                school: "2".to_string(),
                bio: "2".to_string(),
                teacher: "2".to_string(),
            }, vec![Submission {
                id: 3,
                user_id: 2,
                problem_id: 2,
                source_code: "3".to_string(),
                score: 3,
                language: "3".to_string(),
            }])]
        )
    }
}