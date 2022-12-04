export default class Timer {
  time = 0;

  /** start timer */
  public start() {
    this.time = performance.now();
  }

  /** record time elapsed since start */
  public tick(): number {
    return performance.now() - this.time;
  }
}
