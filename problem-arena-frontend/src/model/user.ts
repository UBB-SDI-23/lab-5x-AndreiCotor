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