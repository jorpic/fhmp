// This component fetches a note from storage and presents it for a review.
// There are buttons to mark note as "hard" or "easy".

import {h, Component} from "preact";
import Markdown from "preact-markdown";
import Page from "./Page";
import EditBtn from "./EditBtn";


export default class Review extends Component {
  constructor(props) {
    super(props);
    this.state = {
      queue: null,
      isAnswerVisible: false
    };
  }


  componentDidMount() {
    // FIXME: This will refresh queue on every visit to the review tab.
    // Sometimes this is not the desired behavior. E.g. when we switch tab to
    // edit current note and then come back to review, we possibly don't want
    // to see skipped notes (and we will get them if the queue is refetched).
    this.props.db.getNotesToReview()
      .then(queue => this.setState({queue}))
      .catch(err => this.page.error(
          <span>
            We failed to fetch notes for review. <br />
          </span>
      )(err));
  }


  randomQueue = () =>
    this.props.db.getRandomNotes()
      .then(queue => this.setState({queue}))
      .catch(err => this.page.error(
          <span>
            We failed to fetch notes for review. <br />
          </span>
      )(err))


  showAnswer = () => this.setState({isAnswerVisible: true})


  skip = () => this.setState({
    isAnswerVisible: false,
    queue: this.state.queue.slice(1)
  })


  review = result =>
    this.props.db.addReview(this.state.queue[0], result)
      .then(this.skip)
      .catch(this.page.warning("Failed to save your review"))
  hard = () => this.review("hard")
  easy = () => this.review("easy")


  render() {
    const {queue, isAnswerVisible} = this.state;

    if (queue === null) return (
      <Page ref={ref => this.page = ref}>
        <div class="section">Fetching notes to review…</div>
      </Page>
    );

    if (queue.length === 0) return (
      <Page ref={ref => this.page = ref}>
        <div class="section">
          <p>Nothing to review. Well done!</p>
          <button class="button is-warning" onClick={this.randomQueue}>
            Review random notes
          </button>
        </div>
      </Page>
    );

    const noteId = queue[0].id;
    const [question, answer] = queue[0].text.split(/\n-{4,}\n/);

    return (
      <Page ref={ref => this.page = ref}>
        <div class="edit-btn-container">
          <EditBtn noteId={noteId} />
        </div>
        <div class="content">
          <Markdown markdown={question} />
        </div>
        {!isAnswerVisible &&
          <div class="field">
            <button class="button is-light is-fullwidth" onClick={this.showAnswer}>
              Show the answer
            </button>
          </div>
        }
        {isAnswerVisible &&
          <div class="content">
            <Markdown markdown={answer} />
          </div>
        }
        <nav class="navbar is-light is-fixed-bottom">
          <div class="navbar-brand">
            <Btn text="Hard" onClick={this.hard} />
            <Btn text="Skip" onClick={this.skip} />
            <Btn text="Easy" onClick={this.easy} />
          </div>
        </nav>
      </Page>);
  }
}


function Btn({text, onClick}) {
  return (
    <a class="navbar-item is-expanded" onClick={onClick}>
      {text}
    </a>);
}
