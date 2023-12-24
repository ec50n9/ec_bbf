export class SlidingWindow {
  windowSize: number;
  windowSum: number;
  window: number[];

  constructor(windowSize: number) {
    this.windowSize = windowSize;
    this.windowSum = 0;
    this.window = [];
  }

  addNumber(num: number) {
    this.windowSum += num;
    this.window.push(num);

    if (this.window.length > this.windowSize) {
      const removedNum = this.window.shift()!;
      this.windowSum -= removedNum;
    }
  }

  getAverage() {
    return this.windowSum / this.window.length;
  }
}
