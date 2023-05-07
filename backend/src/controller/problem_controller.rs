use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use actix_web::{web, get, delete, put, post, HttpResponse};
use actix_web::web::{Data, Json, Path, ReqData};
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::model::problem::{NewProblem, Problem, UpdProblem};
use serde::{Deserialize};
use crate::DbPool;
use crate::middleware::authentication_validator;
use crate::model::dto::pagination_dto::{PaginationDTO, StatisticPagination};
use crate::model::dto::problem_dto::{ProblemByOtherSolvedProblemsDTO, ProblemDTO, ProblemStatisticsDTO, ProblemWithCreatorDTO};
use crate::model::dto::token_claims::TokenClaims;
use crate::repository::{problem_repository, submission_repository, user_credentials_repo, users_repo};

#[derive(Deserialize)]
pub struct RatingQuery {
    pub first_id: i32,
    pub last_id: i32,
    pub direction: i32,
    pub limit: i32,
    rating: Option<i32>
}

impl RatingQuery {
    pub fn to_pagination(&self) -> PaginationDTO {
        PaginationDTO {first_id: self.first_id, last_id: self.last_id, direction: self.direction, limit: self.limit }
    }
}

#[derive(Deserialize)]
struct Autocomplete {
    name: Option<String>
}

pub fn problem_config(cfg: &mut web::ServiceConfig) {
    cfg.service(all_problems)
        .service(get_problems_autocomplete)
        .service(problem_number)
        .service(get_problems_by_submissions)
        .service(get_problem_by_id)
        .service(get_problem_number_of_other_problems_solved_by_its_solvers);

}

pub fn problem_restricted(cfg: &mut web::ServiceConfig) {
    cfg.service(add_problem)
        .service(delete_problem)
        .service(update_problem);
}

#[post("/api/problem")]
async fn add_problem(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, new_problem_json: Json<NewProblem>) -> HttpResponse {
    let mut new_problem = new_problem_json.into_inner();
    if !new_problem.is_valid() {
        return HttpResponse::BadRequest().finish();
    }
    
    new_problem.uid = Some(req_user.unwrap().id);

    web::block(move || {
        let mut conn = pool.get().unwrap();
        problem_repository::add_problem(&mut conn, new_problem);
    }).await.unwrap();
    HttpResponse::Ok().finish()
}

#[delete("/api/problem/{id}")]
async fn delete_problem(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, path: Path<i32>) -> HttpResponse {
    let token_data = req_user.unwrap();
    let id = path.into_inner();
    match web::block(move || {
        let mut conn = pool.get().unwrap();

        let problem = problem_repository::get_problem_by_id(&mut conn, id).unwrap().unwrap();
        if token_data.role == "regular" && token_data.id != problem.uid {
            return Err(());
        }

        problem_repository::delete_problem(&mut conn, id);
        Ok(())
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

#[put("/api/problem")]
async fn update_problem(pool: Data<DbPool>, req_user: Option<ReqData<TokenClaims>>, new_problem_json: Json<UpdProblem>) -> HttpResponse {
    let token_data = req_user.unwrap();
    let new_problem = new_problem_json.into_inner();
    if !new_problem.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    match web::block(move || {
        let mut conn = pool.get().unwrap();

        let problem = problem_repository::get_problem_by_id(&mut conn, new_problem.id).unwrap().unwrap();
        if token_data.role == "regular" && token_data.id != problem.uid {
            return Err(());
        }

        problem_repository::update_problem(&mut conn, new_problem);
        Ok(())
    }).await.unwrap() {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

/*#[get("/api/problem")]
async fn all_problems(pool: Data<DbPool>, query: web::Query<RatingQuery>) -> HttpResponse {
    let problems = web::block(move || {
        let mut conn = pool.get().unwrap();
        match query.rating {
            Some(rating) => problem_repository::get_problems_rating_larger(&mut conn, rating),
            None => problem_repository::get_all_problems(&mut conn)
        }
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    HttpResponse::Ok().json(problems)
}*/

#[get("/api/problem/num")]
async fn problem_number(pool: Data<DbPool>) -> HttpResponse {
    let res = web::block(move || {
        let mut conn = pool.get().unwrap();
        problem_repository::number_of_problems(&mut conn).unwrap()
    }).await.unwrap();

    HttpResponse::Ok().json(res)
}

#[get("/api/problem")]
async fn all_problems(pool: Data<DbPool>, query: web::Query<RatingQuery>) -> HttpResponse {
    let mut problems = web::block(move || {
        let mut conn = pool.get().unwrap();
        let problems = match query.rating {
            Some(rating) => problem_repository::get_problems_rating_larger(&mut conn, rating, query.to_pagination()),
            None => problem_repository::get_problems_paginated(&mut conn, query.to_pagination())
        }.unwrap();

        let mut res = vec![];
        for problem in problems {
            let cnt = submission_repository::get_all_submissions_by_problem_id(&mut conn, problem.id).unwrap().len() as i32;
            let creator = user_credentials_repo::get_user_credentials_by_id(&mut conn, problem.uid).unwrap().username;
            res.push(ProblemWithCreatorDTO{problem: problem.clone(), cnt, creator });
        }
        res
    }).await.map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    problems.sort_by(|a, b| a.problem.id.cmp(&b.problem.id));

    HttpResponse::Ok().json(problems)
}

#[get("/api/problem/autocomplete")]
async fn get_problems_autocomplete(pool: Data<DbPool>, query: web::Query<Autocomplete>) -> HttpResponse {
    let mut problems = web::block(move || {
        let mut conn = pool.get().unwrap();
        problem_repository::get_problem_by_name(&mut conn, query.into_inner().name)
    }).await.unwrap().map_err(|_| HttpResponse::InternalServerError().finish()).unwrap();

    HttpResponse::Ok().json(problems)
}

#[get("/api/problem/{id}")]
async fn get_problem_by_id(pool: Data<DbPool>, path: Path<i32>) -> HttpResponse {
    let id = path.into_inner();

    let problem = web::block(move || {
        let mut conn = pool.get().unwrap();
        let problem = problem_repository::get_problem_by_id(&mut conn, id).unwrap().unwrap();
        let submission_list = submission_repository::get_all_submissions_by_problem_id(&mut conn, id).unwrap();
        ProblemDTO {
            problem,
            submissions: submission_list
        }
    }).await.unwrap();

    HttpResponse::Ok().json(problem)
}

#[get("/api/problem-by-submissions")]
async fn get_problems_by_submissions(pool: Data<DbPool>, query: web::Query<StatisticPagination>) -> HttpResponse {
    let mut problems = web::block(move || {
        let mut conn = pool.get().unwrap();
        problem_repository::get_problems_by_submissions(&mut conn, query.into_inner()).unwrap().iter()
            .map(|(problem, cnt)| ProblemStatisticsDTO{problem: problem.clone(), cnt: *cnt})
            .collect::<Vec<ProblemStatisticsDTO>>()
    }).await.unwrap();

    problems.sort_by(|a, b| (a.cnt, a.problem.id).cmp(&(b.cnt, b.problem.id)));

    HttpResponse::Ok().json(problems)
}

#[get("/api/problem-report-solved-by-others")]
async fn get_problem_number_of_other_problems_solved_by_its_solvers(pool: Data<DbPool>) -> HttpResponse {
    let mut problems = web::block(move || {
        let mut conn = pool.get().unwrap();

        let users_with_submissions = users_repo::get_all_users_with_submissions(&mut conn).unwrap();
        let mut submissions_per_user: HashMap<i32, i32> = HashMap::new();
        for user_with_submissions in users_with_submissions {
            submissions_per_user.insert(user_with_submissions.0.id, user_with_submissions.1.len() as i32);
        }

        let problems_with_submissions = problem_repository::get_problems_with_submissions(&mut conn).unwrap();
        let mut result = vec![];
        for problem_with_submission in &problems_with_submissions {
            result.push(ProblemByOtherSolvedProblemsDTO {
                problem: problem_with_submission.0.clone(),
                number_of_other_solved_problems_by_solvers: problem_with_submission.1.iter()
                    .map(|el| el.user_id)
                    .map(|el| submissions_per_user.get(&el).unwrap_or(&0))
                    .sum()
            });
        }

        result
    }).await.unwrap();

    problems.sort_by(|a, b|
        b.number_of_other_solved_problems_by_solvers.cmp(&a.number_of_other_solved_problems_by_solvers));

    HttpResponse::Ok().json(problems)
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test};
    use actix_web::http::header::ContentType;
    use actix_web::web::Data;
    use diesel::PgConnection;
    use diesel::r2d2::ConnectionManager;
    use crate::controller::problem_controller::get_problems_statistics;
    use crate::model::dto::problem_dto::ProblemStatisticsDTO;
    use crate::model::problem::Problem;
    use crate::utils::mock::MockablePool;
    use actix_web::body::MessageBody;

    #[actix_web::test]
    async fn test_get_problems_statistics() {
        let app =test::init_service(App::new()
            .app_data(Data::new(MockablePool::<ConnectionManager<PgConnection>>::Mock))
            .service(get_problems_statistics))
            .await;

        let req = test::TestRequest::with_uri("/problem/report")
            .insert_header(ContentType::json())
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let interm = &resp.into_body().try_into_bytes().unwrap()[..];
        let body = std::str::from_utf8(interm).unwrap();

        let expected = vec![
            ProblemStatisticsDTO {
                problem: Problem {
                    id: 2,
                    name: "2".to_string(),
                    author: "2".to_string(),
                    contest: "2".to_string(),
                    statement: "2".to_string(),
                    rating: 2,
                },
                success_rate: Some(0.5)
            },
            ProblemStatisticsDTO {
                problem: Problem {
                    id: 1,
                    name: "1".to_string(),
                    author: "1".to_string(),
                    contest: "1".to_string(),
                    statement: "1".to_string(),
                    rating: 0,
                },
                success_rate: Some(1.0/3.0),
            }
        ];

        assert_eq!(body, serde_json::to_string(&expected).unwrap());
    }
}