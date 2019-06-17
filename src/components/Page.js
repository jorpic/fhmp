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
  message = message => {
    if (message.error) console.error(message);
    else if (message.warning) console.warn(message);
    this.setState({message});
  }

  success = msg => () => this.message({success: true, msg})
  warning = msg => err => this.message({warning: true, err, msg})
  error = msg => err => this.message({error: true, err, msg})


  clearMessage = () => this.setState({message: null})


  render() {
    const {message} = this.state;
    return (
      <div>
        <Navbar />
        <div class={cls("section", this.props.class)}>
          {this.props.children}
        </div>
        {message &&
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
