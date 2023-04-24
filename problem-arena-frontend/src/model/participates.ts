import {User} from "./user";
import {Contest} from "./contest";

export interface Participation {
    uid: number,
    cid: number,
    score: number,
    official: boolean
}

export interface ParticipationDTO {
    uid: number,
    cid: number,
    score: number,
    official: boolean,
    user: User,
    contest: Contest
}

export interface ParticipationWithName {
    uid: number,
    cid: number,
    score: number,
    official: boolean,
    user: string,
    contest: string
}