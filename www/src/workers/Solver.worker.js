import { Solver } from './Solver.js';

{
  var solver = new Solver(self);

  self.onmessage = (evt) => {
    if (evt.data instanceof Object) {
      if (evt.data.make) {
        solver.set_parameters(evt.data.make);
        solver.initialize();
        solver.send_solution();

      } else if (evt.data.update) {
        solver.set_parameters(evt.data.parameters);

      } else if (evt.data.start) {
        solver.start();

      } else if (evt.data.stop) {
        solver.stop();

      } else if (evt.data.get) {
        solver.send_solution();

      } else {
        throw "Event " + JSON.stringify(evt.data) + " is not supported";
      }
    }
  };
}
  
