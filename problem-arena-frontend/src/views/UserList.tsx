import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import Table from "../components/Table";
import {PaginationDTO} from "../model/PaginationDTO";
import {UserSubmissionsDTO} from "../model/user";
import {UserService} from "../services/user-service";

export default function UserList() {
    const [userList, setUserList] = useState<UserSubmissionsDTO[]>([]);
    const [value, setValue] = useState<number>(0);
    const [pagination, setPagination] = useState<PaginationDTO>({first_id: -1, last_id: 0, limit: 10, direction: 1});
    const navigate = useNavigate();

    useEffect(() => {
        UserService.getUsers(pagination).then((res) => {
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
            setPagination({first_id: userList[0].id, last_id: userList[userList.length - 1].id, limit: 10, direction: -1});
        }
    }

    const nextPage = () => {
        if (userList.length > 0) {
            setPagination({first_id: userList[0].id, last_id: userList[userList.length - 1].id, limit: 10, direction: 1});
        }
    }

    return (
        <div className="mr-2">
            <div className="columns">
                <div className="column">
                    <h1 className="title">User List</h1>
                </div>
            </div>
            <Table columns={["First Name", "Last Name", "School", "Teacher", "Submissions"]}
                   properties={["first_name", "last_name", "school", "teacher", "cnt"]}
                   elements={userList}
                   path={"/user"}
                   deleteFunction={(id) => deleteUser(id)}
            />
            <nav className="pagination" role="navigation" aria-label="pagination">
                <button className="pagination-previous" onClick={() => previousPage()}>Previous</button>
                <button className="pagination-next" onClick={() => nextPage()}>Next page</button>
            </nav>
        </div>
    );
}