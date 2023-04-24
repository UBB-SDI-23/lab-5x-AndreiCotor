import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import {NewProblem, Problem, ProblemStatisticsDTO} from "../model/problem";
import {PaginationDTO, StatisticPagination} from "../model/PaginationDTO";

export const ProblemsService = {
    getProblems: (pagination: PaginationDTO, filter: number | undefined): Promise<AxiosResponse<ProblemStatisticsDTO[]>> => {

        return axiosConfigured.get("/problem", {params: {
                first_id: pagination.first_id,
                last_id: pagination.last_id,
                direction: pagination.direction,
                limit: pagination.limit,
                rating: filter
        }});
    },

    getProblem: (id: string): Promise<AxiosResponse<Problem>> => {
        return axiosConfigured.get("/problem/" + id);
    },

    updateProblem: (problem: Problem): Promise<AxiosResponse> => {
        return axiosConfigured.put("/problem", problem);
    },

    addProblem: (problem: NewProblem): Promise<AxiosResponse> => {
        return axiosConfigured.post("/problem", problem);
    },

    deleteProblem: (id: string): Promise<AxiosResponse> => {
        return axiosConfigured.delete("/problem/" + id);
    },

    getProblemsBySuccessRate: (pagination: StatisticPagination): Promise<AxiosResponse<ProblemStatisticsDTO[]>> => {
        return axiosConfigured.get("/problem-by-submissions", {params: pagination});
    },

    getProblemsAutocomplete: (name: string): Promise<AxiosResponse<Problem[]>> => {
        return axiosConfigured.get("/problem/autocomplete", {params: {name}})
    },

    getNumberOfProblems: (): Promise<AxiosResponse<number>> => {
        return axiosConfigured.get("/problem/num");
    },
}