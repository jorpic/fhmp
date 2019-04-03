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
    return (
      <div class="container">
          <div class="buttons has-addons is-right">
            <button class="button is-light" onClick={this.skip}>Skip</button>
            <button class="button is-danger" onClick={this.soon}>Review soon</button>
            <button class="button is-warning" onClick={this.later}>Review later</button>
            <button class="button is-success" onClick={this.never}>Review occasionally</button>
          </div>
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
      </div>
    );
  }
}
