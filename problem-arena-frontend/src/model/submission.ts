import {User} from "./user";
import {Problem} from "./problem";

export interface Submission {
    id: number,
    user_id: number,
    problem_id: number,
    source_code: string,
    score: number,
    language: string
}

export interface NewSubmission {
    user_id: number,
    problem_id: number,
    source_code: string,
    score: number,
    language: string
}

export interface SubmissionDTO {
    id: number,
    source_code: string,
    score: number,
    language: string,
    user: User,
    problem: Problem
}