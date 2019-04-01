import cls from "classnames";
import { h, Component } from "preact";
import Markdown from "preact-markdown";


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
    .then(n => this.setState({
      noteId: n.id,
      question: n.text,
      answer: n.createTime,
      isAnswerVisible: false,
    }))
  soon = () => {
    this.props.updNote({}); // this.state.noteId, {reviewTimestamp.push(now), result: -1});
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
