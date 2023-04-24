export default function NavMenu() {
    return (
        <aside className="menu">
            <p className="menu-label">General</p>
            <ul className="menu-list">
                <li><a href="/">Home</a></li>
                <li><a href="/problems">Problem Archive</a></li>
                <li><ul>
                    <li><a href="/problems">All problems</a></li>
                    <li><a href="/problems-by-success-rate">Problems increasing by submissions</a></li>
                </ul></li>
                <li><a href="/contests">Training Contests</a></li>
                <li><a href="/submissions">Submissions</a></li>
                <li><a href="/users">Users</a></li>
                <li><ul>
                    <li><a href="/users">All users</a></li>
                    <li><a href="/users-by-participation">Users increasing by participations</a></li>
                </ul></li>
                <li><a href="/participations">Participations</a></li>
            </ul>
        </aside>
    );
}