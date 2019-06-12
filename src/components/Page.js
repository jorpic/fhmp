// Page wrapper adds navigation bar and is able to handle notifications.

import cls from "classnames";
import {h, cloneElement, Component} from "preact";
import Navbar from "./Navbar";


export default class Page extends Component {
  constructor(props) {
    super(props);
    this.state = {
      message: null
    };
  }


  // Show important messages on a modal form.
  onMessage = message => {
    if (message.error) console.error(message);
    else if (message.warning) console.warn(message);
    this.setState({message});
  }

  clearMessage = () => this.setState({message: null})


  render() {
    const pageContents = this.props.children.map(i =>
      i && i.nodeName
        ? cloneElement(i, {onMessage: this.onMessage})
        : i);

    return (
      <div>
        <Navbar />
        <div class={cls("section", this.props.class)}>
          {pageContents}
        </div>
        {this.state.message &&
          <div class="modal is-active">
            <div class="modal-background" onClick={this.clearMessage} />
            <div class="modal-content" onClick={this.clearMessage}>
              <article
                class={cls("message", {
                  "is-danger": message.error,
                  "is-warning": message.warning,
                  "is-success": message.success})}
              >
                <div class="message-body">{message.msg}</div>
              </article>
            </div>
          </div>
        }
      </div>);
  }
}
