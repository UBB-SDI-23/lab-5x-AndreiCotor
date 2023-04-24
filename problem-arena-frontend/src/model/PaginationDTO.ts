export interface PaginationDTO {
    first_id: number,
    last_id: number,
    direction: number,
    limit: number
}

export interface ParticipationPaginationDTO {
    first_uid: number,
    first_cid: number,
    last_uid: number,
    last_cid: number,
    direction: number,
    limit: number
}

export interface StatisticPagination {
    first_stat: number,
    first_id: number,
    last_stat: number,
    last_id: number,
    direction: number,
    limit: number
}