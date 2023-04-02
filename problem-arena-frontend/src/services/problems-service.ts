import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import {NewProblem, Problem} from "../model/problem";

export const ProblemsService = {
    getProblems: (): Promise<AxiosResponse<Problem[]>> => {
        return axiosConfigured.get("/problem");
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
    }
}