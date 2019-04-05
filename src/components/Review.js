import { h, Component } from "preact";
import Markdown from "preact-markdown";


// TODO: gently handle "unable to save review" (this is not a critical error)

export default class Review extends Component {
  constructor(props) {
    super(props);
    this.state = {
      question: "",
      answer: "",
      isAnswerVisible: false
    };
    this.skip()
  }


  showAnswer = () => this.setState({isAnswerVisible: true})

  skip = () => this.props.getNote()
    .then(n => {
      const [question, answer] = n.text.split(/\n-{4,}\n/);
      return this.setState({
        noteId: n.id,
        question,
        answer,
        isAnswerVisible: false,
      })
    })
  soon = () => {
    this.props.updateNote({}); // this.state.noteId, {reviewTimestamp.push(now), result: -1});
    this.skip();
  }
  later = () => this.soon()
  never = () => this.soon()


  render() {
    const btn = (text, onClick) =>
      <a class="navbar-item is-expanded" onClick={onClick}>
        {text}
      </a>;

    return (
      <div class="section">
        <div class="content">
          <Markdown markdown={this.state.question}/>
        </div>
        {!this.state.isAnswerVisible &&
          <div class="field">
            <button class="button is-light is-fullwidth" onClick={this.showAnswer}>
              Show
            </button>
          </div>
        }
        {this.state.isAnswerVisible &&
          <div class="content">
            <Markdown markdown={this.state.answer}/>
          </div>
        }
        <nav class="navbar is-light is-fixed-bottom">
          <div class="navbar-brand">
            {btn("Skip", this.skip)}
            {btn("Soon", this.soon)}
            {btn("Later", this.later)}
            {btn("Occasinally", this.never)}
          </div>
        </nav>
      </div>
    );
  }
}
