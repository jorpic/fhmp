import cls from "classnames";
import {h, Component} from "preact";
import Markdown from "preact-markdown";
import {dateToString, default as config} from "../config";
import Page from "./Page";
import EditBtn from "./EditBtn";


export default class ListNotes extends Component {
  constructor(props) {
    super(props);
    this.state = {
      notes: [],
      expand: null,
    };
  }


  componentDidMount() {
    const transform = n => {
      const [question, answer] = n.text.split(/\n-{4,}\n/);
      return {question, answer, ...n}
    };
    this.props.db.getNotes()
      .then(ns => this.setState({notes: ns.map(transform)}))
      .catch(this.page.error("Failed to load notes from local storage."));
  }


  expand = id => this.setState({
    expand: id === this.state.expand ? null : id,
  })


  render() {
    return (
      <Page ref={ref => this.page = ref}>
        {this.state.notes.map(n =>
          <article
            class="notification edit-btn-container is-clickable"
            onClick={() => this.expand(n.id)}
          >
            <span>{n.question}</span>
            {n.id == this.state.expand && (
              <div class="content">
                <div>
                  <strong>Last review:&nbsp;</strong>
                  {dateToString(n.lastReview)}
                </div>
                <div>
                  <strong>Next review:&nbsp;</strong>
                  {dateToString(n.nextReview)}
                </div>
                <Markdown markdown={n.answer} />
              </div>
            )}
            <EditBtn class="is-light" noteId={n.id} />
          </article>
        )}
      </Page>
    );
  }
}
