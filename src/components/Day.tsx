import { h } from "preact";

export interface DayProps {
  part1: string;
  part2: string;
  execTime: number;
}

export default function Day({ part1, part2, execTime }: DayProps) {
  return (
    <div class="day">
      <dl class="day-solutions">
        <dt>Part 1</dt>
        <dd>{part1}</dd>
        <dt>Part 2</dt>
        <dd>{part2}</dd>
      </dl>
      <div class="day-exec">
        Executed in <code>{execTime.toFixed(2)}ms</code>
      </div>
    </div>
  );
}
