import cls from "classnames";
import {h, Component} from "preact";
import {route} from "preact-router";
import config from "../config";
import Page from "./Page";


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


  onEdit(noteId) {
    return () => route("/edit/" + noteId)
  }


  render() {
    return (
      <Page class="list-notes">
        {this.state.notes.map(n =>
          <article class="notification">
            {n.question}
              <span
                class="button edit is-small is-light"
                onClick={this.onEdit(n.id)}
              >
                <i class="fas fa-edit" />
              </span>
          </article>
        )}
      </Page>
    );
  }
}
