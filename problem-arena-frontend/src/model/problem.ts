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
    success_rate: number | null
}