import {useNavigate} from "react-router-dom";
import RatingDisplay from "./RatingDisplay";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPenToSquare, faTrash} from "@fortawesome/free-solid-svg-icons";

interface TableProps {
    columns: string[],
    properties: string[],
    elements: any[],
    path: string,
    creator?: string,
    uid?: string,
    user?: string,
    deleteFunction: (id: string) => void
}

export default function Table(props: TableProps) {
    const navigate = useNavigate();

    function extractObjectProperties(properties: string[], obj: any) {
        let tsx_list = properties.map((prop) => {
            return (
                <td onClick={() => navigate(props.path + "/" + obj.id)}>
                    { prop === "rating"? (<RatingDisplay rating={obj[prop]}/>) : obj[prop] }
                </td>
            );

        });

        if (props.creator && props.uid) {
            tsx_list.push(
                <td>
                    <a href={"/user/" + obj[props.uid]}>{obj[props.creator]}</a>
                </td>
            );
        }

        return tsx_list;
    }



    const tableRows = props.elements.map((el, index) => {
        return (<tr key={index}>
            <td onClick={() => navigate(props.path + "/" + el.id)}>{index + 1}</td>
            { extractObjectProperties(props.properties, el) }
            <td>
                <button className="button is-danger" onClick={() => props.deleteFunction(String(el.id))}>
                    <FontAwesomeIcon icon={faTrash} />
                </button>
                <button className="button is-link ml-2" onClick={() => navigate(props.path + "/edit/" + el.id)}>
                    <FontAwesomeIcon icon={faPenToSquare} />
                </button>
            </td>
        </tr>);
    });

    const tableHeader = props.columns.map((el) => {
        return (
            <th>{ el }</th>
        );
    });

    if ((props.creator && props.uid) || props.user) {
        tableHeader.push(<th>Username</th>)
    }

    return (
        <div>
            <table className="table is-hoverable is-fullwidth ">
                <thead>
                <tr>
                    <th><abbr title="Index">#</abbr></th>
                    {tableHeader}
                    <th>Actions</th>
                </tr>
                </thead>
                <tbody>
                    {tableRows}
                </tbody>
            </table>
        </div>
    );
}