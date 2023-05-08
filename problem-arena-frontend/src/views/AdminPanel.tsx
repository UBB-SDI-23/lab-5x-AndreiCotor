import {useState} from "react";
import {AdminService} from "../services/admin-service";

export default function AdminPanel() {
    const [info, setInfo] = useState<string>("");
    const [active, setActive] = useState<boolean>(true);

    const submit = (table: string) => {
        AdminService.deleteAllFromTable(table).then((res) => {
            if (res.status) {
                setInfo("Table deleted successfully");
            }
            else {
                setInfo("An error has occurred");
            }
        }).catch((res) => {
            setInfo("An error has occurred");
        })
    }

    const populate = () => {
        setActive(false);
        setInfo("Running database populate scripts. This may take a while...")
    }

    return (
        <div>
            <h1 className="title">Admin panel</h1>
            <div className="columns">
                <div className="column is-half-desktop">
                    <p>{info}</p>
                    <button className="button is-link mr-2" onClick={() => submit("participates")}>
                        Delete all participations
                    </button>
                    <button className="button is-link mr-2" onClick={() => submit("submissions")}>
                        Delete all submissions
                    </button>
                    <button className="button is-link mr-2" onClick={() => submit("contests")}>
                        Delete all contests
                    </button>
                    <button className="button is-link mr-2" onClick={() => submit("problems")}>
                        Delete all problems
                    </button>
                    <button className="button is-link mt-2" onClick={() => populate()} disabled={!active}>
                        Run populate script
                    </button>
                </div>
            </div>
        </div>
    );
}