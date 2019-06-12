import cls from "classnames";
import {h, Component} from "preact";
import config from "../config";
import Page from "./Page";
import EditBtn from "./EditBtn";


export default class ListNotes extends Component {
  constructor(props) {
    super(props);
    this.state = {
      notes: []
    };
  }


  componentDidMount() {
    const transform = n => {
      const [question, answer] = n.text.split(/\n-{4,}\n/);
      return {question, answer, id: n.id};
    };
    this.props.db.getNotes()
      .then(ns => this.setState({notes: ns.map(transform)}))
      .catch(err => this.props.onMessage({
        error: true,
        err,
        msg: "Failed to load notes from local storage."
      }));
  }


  render() {
    return (
      <Page>
        {this.state.notes.map(n =>
          <article class="notification edit-btn-container">
            {n.question}
            <EditBtn class="is-light" noteId={n.id} />
          </article>
        )}
      </Page>
    );
  }
}
