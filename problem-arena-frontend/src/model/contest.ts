export interface Contest {
    id: number
    name: string
    description: string
}

export interface NewContest {
    name: string
    description: string
}

export interface ContestDTO {
    id: number
    name: string
    description: string
    cnt: number
}

export interface ContestWithCreatorDTO {
    id: number
    name: string
    description: string
    cnt: number
    uid: number
    creator: string
}
