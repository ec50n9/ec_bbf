import { createAlovaMockAdapter } from "@alova/mock";
import GlobalFetch from "alova/GlobalFetch";
import scoreMock from "@/mock/score";
import studentMock from "@/mock/student";
import studentGroupMock from "@/mock/student-group";

const tauriMockAdapter = createAlovaMockAdapter(
  [scoreMock, studentMock, studentGroupMock],
  {
    enable: true,
    httpAdapter: GlobalFetch(),
    delay: 300,
    mockRequestLogger: true,
  }
);

export default tauriMockAdapter;
