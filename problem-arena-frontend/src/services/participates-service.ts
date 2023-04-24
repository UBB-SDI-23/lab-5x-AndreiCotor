import {axiosConfigured} from "../config";
import {AxiosResponse} from "axios";
import {ParticipationPaginationDTO} from "../model/PaginationDTO";
import {Participation, ParticipationDTO} from "../model/participates";

export const ParticipationService = {
    getParticipations: (pagination: ParticipationPaginationDTO): Promise<AxiosResponse<ParticipationDTO[]>> => {
        return axiosConfigured.get("/participates", {params: pagination});
    },

    getParticipation: (id: string): Promise<AxiosResponse<Participation>> => {
        return axiosConfigured.get("/participates/" + id);
    },

    updateParticipation: (participation: Participation): Promise<AxiosResponse> => {
        return axiosConfigured.put("/participates", participation);
    },

    addParticipation: (participation: Participation): Promise<AxiosResponse> => {
        return axiosConfigured.post("/participates", [participation]);
    },

    deleteParticipation: (id: string): Promise<AxiosResponse> => {
        return axiosConfigured.delete("/participates/" + id);
    }
}