import cls from "classnames";
import {h, cloneElement, Component} from "preact";


export class Navbar extends Component {
  constructor(props) {
    super(props);
    this.state = {
      isMenuActive: false,
    };
  }


  toggleBurger = () => this.setState({
    isMenuActive: !this.state.isMenuActive
  })


  render() {
    const isActiveCls = cls({"is-active": this.state.isMenuActive});
    const brand = this.props.children[0];
    let   items = this.props.children.slice(1);

    // override menuItem's onClick handler to hide dropdown
    items = items.map(i => {
      const onClick = ev => {
        this.setState({isMenuActive: false});
        i.attributes && i.attributes.url && this.props.onChange(i.attributes.url);
        return i.attributes && i.attributes.onClick && i.attributes.onClick(ev);
      };
      const isActive = i.attributes && i.attributes.url === this.props.url;
      return cloneElement(i, {onClick, isActive});
    });

    return (
      <nav class="navbar has-shadow is-fixed-top"
        role="navigation" aria-label="main navigation"
      >
        <div class="navbar-brand">
          <div class="navbar-item">{brand}</div>
          <a class={cls("navbar-burger burger", isActiveCls)}
            role="button" aria-label="menu" aria-expanded="false"
            onClick={this.toggleBurger}
          >
            <span aria-hidden="true" />
            <span aria-hidden="true" />
            <span aria-hidden="true" />
          </a>
        </div>

        <div class={cls("navbar-menu", isActiveCls)}>
          <div class="navbar-start">
            {items}
          </div>
        </div>
      </nav>
    );
  }
}


export const NavbarItem = props => (
  <a class={cls("navbar-item", {"is-active": props.isActive})}
    onClick={props.onClick}
  >
    {props.icon && <span class="icon"><i class={props.icon} /></span>}
    <span>{props.text}</span>
  </a>
);
