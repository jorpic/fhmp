import cls from "classnames";
import { h, Component } from "preact";

export class Tabs extends Component {
  constructor(props) {
    super(props);
    this.state = {
      active: 0
    };
  }

  render() {
    const mkTab = (tab, i) => (
      <li onClick={() => this.setState({active: i})}
          className={cls({"is-active": this.state.active == i})}>
        <a>{tab.attributes.name}</a>
      </li>);
    const { children } = this.props;

    return (
      <div className="tabs-wrapper">
        <div className="tabs is-centered">
          <ul>
            {children.map(mkTab)}
          </ul>
        </div>
        {children[this.state.active]}
      </div>);
  }
}


export const Tab = ({children}) =>
    children.length == 1
      ? children[0]
      : (<div>{children}</div>);
