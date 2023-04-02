import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import Problem from "../model/problem";

export const ProblemsService = {
    getProblems: (): Promise<AxiosResponse<Problem[]>> => {
        return axiosConfigured.get("/problem");
    },

    getProblem: (id: string): Promise<AxiosResponse<Problem>> => {
        return axiosConfigured.get("/problem/" + id);
    }
}