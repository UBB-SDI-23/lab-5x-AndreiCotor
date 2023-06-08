import {useEffect, useState} from "react";
import Table from "../components/Table";
import {StatisticPagination} from "../model/PaginationDTO";
import {UserReportDTO} from "../model/user";
import {UserService} from "../services/user-service";

export default function UserListByParticipations() {
    const [userList, setUserList] = useState<UserReportDTO[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<StatisticPagination>({first_id: -1, first_stat: -1, last_id: 0, last_stat: 0, limit: 10, direction: 1});

    useEffect(() => {
        UserService.getProblemsByParticipations(pagination).then((res) => {
            if (res.data.length > 0) {
                setUserList(res.data);
            }
        })
    }, [value, pagination]);

    function forceUpdate() {
        setValue(value => value + 1);
    }

    const deleteUser = async (id: string) => {
        if (window.confirm("Are you sure you want to delete this entry?")) {
            await UserService.deleteUser(id);
            forceUpdate();
        }
    }

    const previousPage = () => {
        if (userList.length > 0) {
            setPagination({
                first_id: userList[0].id,
                first_stat: userList[0].participations,
                last_id: userList[userList.length - 1].id,
                last_stat: userList[userList.length - 1].participations,
                limit: 10,
                direction: -1
            });
        }
    }

    const nextPage = () => {
        if (userList.length > 0) {
            setPagination({
                first_id: userList[0].id,
                first_stat: userList[0].participations,
                last_id: userList[userList.length - 1].id,
                last_stat: userList[userList.length - 1].participations,
                limit: 10,
                direction: 1
            });
        }
    }

    const firstPage = () => {
        setPagination({first_id: -1, first_stat: -1, last_id: 0, last_stat: 0, limit: 10, direction: 1});
    }

    const lastPage = () => {
        setPagination({first_id: 1000000000, first_stat: 1000000000, last_id: 1000000000, last_stat: 1000000000, limit: 10, direction: -1});
    }

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">User List</h1>
                </div>
            </div>
            <Table columns={["First Name", "Last Name", "School", "Teacher", "Participations"]}
                   properties={["first_name", "last_name", "school", "teacher", "participations"]}
                   elements={userList}
                   path={"/user"}
                   deleteFunction={(id) => deleteUser(id)}
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
                <ul className="pagination-list">
                    <button className="pagination-link" onClick={() => firstPage()}>First page</button>
                    <button className="pagination-link" onClick={() => lastPage()}>Last page</button>
                </ul>
            </nav>
        </div>
    );
}