export interface User {
    id: number,
    first_name: string,
    last_name: string,
    school: string,
    bio: string,
    teacher: string
}

export interface NewUser {
    first_name: string,
    last_name: string,
    school: string,
    bio: string,
    teacher: string
}

export interface UserReportDTO {
    id: number,
    first_name: string,
    last_name: string,
    school: string,
    bio: string,
    teacher: string,
    participations: number
}

export interface UserSubmissionsDTO {
    id: number,
    first_name: string,
    last_name: string,
    school: string,
    bio: string,
    teacher: string,
    cnt: number
}

export interface UserPageDTO {
    id: number,
    first_name: string,
    last_name: string,
    school: string,
    bio: string,
    teacher: string,
    username: string,
    problems_proposed: number,
    contests_created: number,
    submissions_sent: number,
    participations: number,
    role: string
}