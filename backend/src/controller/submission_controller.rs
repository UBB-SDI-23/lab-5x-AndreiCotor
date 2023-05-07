use actix_web::{HttpResponse, web, post, get, delete, put};
use actix_web::web::{Data, Json, Path};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::DbPool;
use crate::middleware::authentication_validator;
use crate::model::dto::pagination_dto::PaginationDTO;
use crate::model::dto::submission_dto::{SubmissionDTO, SubmissionReportDTO};
use crate::model::submission::{NewSubmission, Submission};
use crate::repository::{problem_repository, submission_repository, users_repo};

pub fn submission_config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_submissions)
        .service(get_submission)
        .service(get_submission_by_other_submissions_its_user_created);
}

pub fn submission_restricted(cfg: &mut web::ServiceConfig) {
    cfg.service(add_submission)
        .service(delete_submission)
        .service(update_submission);
}

#[post("/api/submission")]
async fn add_submission(pool: Data<DbPool>, new_submission_json: Json<NewSubmission>) -> HttpResponse {
    let new_submission = new_submission_json.into_inner();
    if !new_submission.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    web::block(move || {
        let mut conn = pool.get().unwrap();
        submission_repository::add_submission(&mut conn, new_submission);
    }).await.unwrap();
    HttpResponse::Ok().finish()
}

#[get("/api/submission")]
async fn all_submissions(pool: Data<DbPool>, query: web::Query<PaginationDTO>) -> HttpResponse {
    let mut submissions = web::block(move || {
        let mut conn = pool.get().unwrap();
        let submissions = submission_repository::get_submissions_paginated(&mut conn, query.into_inner()).unwrap();

        let mut res = vec![];
        for submission in submissions {
            let user = users_repo::get_user_by_id(&mut conn, submission.user_id).unwrap().unwrap();
            let problem = problem_repository::get_problem_by_id(&mut conn, submission.problem_id).unwrap().unwrap();
            res.push(SubmissionDTO{submission, user, problem});
        }
        res
    }).await.map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    submissions.sort_by(|a, b| a.submission.id.cmp(&b.submission.id));

    HttpResponse::Ok().json(submissions)
}

#[get("/api/submission/{id}")]
async fn get_submission(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let submission = web::block(move || {
        let mut conn = pool.get().unwrap();
        let submission = submission_repository::get_submission(&mut conn, id).unwrap().unwrap();
        let user = users_repo::get_user_by_id(&mut conn, submission.user_id).unwrap().unwrap();
        let problem = problem_repository::get_problem_by_id(&mut conn, submission.problem_id).unwrap().unwrap();
        SubmissionDTO {
            submission, user, problem
        }
    }).await.unwrap();

    HttpResponse::Ok().json(submission)
}

#[put("/api/submission")]
async fn update_submission(pool: Data<DbPool>, submission_json: Json<Submission>) -> HttpResponse {
    let submission = submission_json.into_inner();
    if !submission.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    let val = web::block(move || {
        let mut conn = pool.get().unwrap();
        submission_repository::update_submission(&mut conn, submission)
    }).await.unwrap();

    match val {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/api/submission/{id}")]
async fn delete_submission(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let val = web::block(move || {
        let mut conn = pool.get().unwrap();
        submission_repository::delete_submission(&mut conn, path.into_inner())
    }).await.unwrap();

    match val {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/api/submission-by-other-submissions-its-users-created")]
async fn get_submission_by_other_submissions_its_user_created(pool: Data<DbPool>) -> HttpResponse {
    let mut submissions = web::block(move || {
        let mut conn = pool.get().unwrap();
        let users_subs =users_repo::get_all_users_with_submissions(&mut conn).unwrap();

        let mut result = vec![];
        for user_sub in users_subs {
            let number_of_other_submissions = user_sub.1.len() as i32 - 1;
            for submission in &user_sub.1 {
                result.push(SubmissionReportDTO{
                    submission: submission.clone(),
                    number_of_other_submissions
                })
            }
        }

        result
    }).await.unwrap();

    submissions.sort_by(|a, b| b.number_of_other_submissions.cmp(&a.number_of_other_submissions));

    HttpResponse::Ok().json(submissions)
}

#[cfg(test)]
mod submission_tests {
    use actix_web::{App, test, web, http::header::ContentType, web::Data};
    use diesel::PgConnection;
    use diesel::r2d2::ConnectionManager;
    use serde_json::json;
    use crate::controller::submission_controller::get_submission_by_other_submissions_its_user_created;
    use crate::utils::mock::MockablePool;
    use crate::model::dto::submission_dto::SubmissionReportDTO;
    use crate::model::submission::Submission;
    use actix_web::body::MessageBody;


    #[actix_web::test]
    async fn test_get_submission_by_other_submissions_its_user_created() {
        let app =test::init_service(App::new()
            .app_data(Data::new(MockablePool::<ConnectionManager<PgConnection>>::Mock))
            .service(get_submission_by_other_submissions_its_user_created))
            .await;

        let req = test::TestRequest::with_uri("/submission-report")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let interm = &resp.into_body().try_into_bytes().unwrap()[..];
        let body = std::str::from_utf8(interm).unwrap();
        let expected = vec![
            SubmissionReportDTO {
                submission: Submission {
                    id: 1,
                    user_id: 1,
                    problem_id: 1,
                    source_code: "1".to_string(),
                    score: 1,
                    language: "1".to_string(),
                },
                number_of_other_submissions: 1,
            },
            SubmissionReportDTO {
                submission: Submission {
                    id: 2,
                    user_id: 1,
                    problem_id: 2,
                    source_code: "2".to_string(),
                    score: 2,
                    language: "2".to_string(),
                },
                number_of_other_submissions: 1,
            },
            SubmissionReportDTO {
                submission: Submission {
                    id: 3,
                    user_id: 2,
                    problem_id: 2,
                    source_code: "3".to_string(),
                    score: 3,
                    language: "3".to_string(),
                },
                number_of_other_submissions: 0,
            }
        ];

        assert_eq!(body, serde_json::to_string(&expected).unwrap());
    }

}