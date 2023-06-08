export interface Problem {
    id: number
    name: string
    author: string
    contest: string
    statement: string
    rating: number
}

export interface NewProblem {
    name: string
    author: string
    contest: string
    statement: string
    rating: number
}

export interface ProblemStatisticsDTO {
    id: number
    name: string
    author: string
    contest: string
    statement: string
    rating: number,
    uid: number,
    cnt: number
}

export interface ProblemWithCreatorDTO {
    id: number
    name: string
    author: string
    contest: string
    statement: string
    rating: number,
    uid: number,
    cnt: number
    creator: string
}