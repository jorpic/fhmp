// This component fetches a note from storage and presents it for a review.
// There are buttons to mark note as "hard" or "easy".
// TODO:
//  - gently handle "unable to save review" (this is not a critical error)
//  - show number of notes in the review queue
//  - add a button to fetch random notes if queue is empty
//  - add a button to edit current note
//  - desktop-friendly styles for review buttons

import {h, Component} from "preact";
import Markdown from "preact-markdown";


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
      .catch(err => this.props.onMessage({
        error: true,
        err,
        msg: (
          <span>
            We are failed to fetch notes for review. <br />
            {err}
          </span>)
      }));
  }


  showAnswer = () => this.setState({isAnswerVisible: true})


  skip = () => this.setState({
    isAnswerVisible: false,
    queue: this.state.queue.slice(1)
  })


  hard = () => console.log("not implemented")
  easy = () => console.log("not implemented")


  render() {
    const {queue, isAnswerVisible} = this.state;

    if (queue === null) {
      return (<div class="section">Fetching notes to review…</div>);
    }

    if (queue.length === 0) {
      return (<div class="section">Nothing to review. Well done!</div>);
    }

    const [question, answer] = queue[0].text.split(/\n-{4,}\n/);

    return (
      <div class="section">
        <div class="content"><Markdown markdown={question} /></div>
        {!isAnswerVisible &&
          <div class="field">
            <button class="button is-light is-fullwidth" onClick={this.showAnswer}>
              Show
            </button>
          </div>
        }
        {isAnswerVisible &&
          <div class="content"><Markdown markdown={answer} /></div>
        }
        <nav class="navbar is-light is-fixed-bottom">
          <div class="navbar-brand">
            <Btn text="Hard" onClick={this.hard} />
            <Btn text="Skip" onClick={this.skip} />
            <Btn text="Easy" onClick={this.easy} />
          </div>
        </nav>
      </div>);
  }
}


function Btn({text, onClick}) {
  return (
    <a class="navbar-item is-expanded" onClick={onClick}>
      {text}
    </a>);
}
