import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import {PaginationDTO} from "../model/PaginationDTO";
import {Contest, ContestDTO, NewContest} from "../model/contest";

export const ContestService = {
    getContests: (pagination: PaginationDTO): Promise<AxiosResponse<ContestDTO[]>> => {
        return axiosConfigured.get("/contest", {params: pagination});
    },

    getContest: (id: string): Promise<AxiosResponse<Contest>> => {
        return axiosConfigured.get("/contest/" + id);
    },

    updateContest: (contest: Contest): Promise<AxiosResponse> => {
        return axiosConfigured.put("/contest", contest);
    },

    addContest: (contest: NewContest): Promise<AxiosResponse> => {
        return axiosConfigured.post("/contest", contest);
    },

    deleteContest: (id: string): Promise<AxiosResponse> => {
        return axiosConfigured.delete("/contest/" + id);
    },

    getContestAutocomplete: (name: string): Promise<AxiosResponse<Contest[]>> => {
        return axiosConfigured.get("/contest/autocomplete", {params: {name}})
    }
}