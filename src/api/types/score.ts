export type ScoreType = {
  id?: string;
  name: string;
  desc: string;
  icon?: string;
  color?: string;
};

export type ScoreTypeCreateVO = {
  name: string;
  desc: string;
  icon?: string;
  color?: string;
};

export type ScoreTypeUpdateVO = {
  id: string;
  name?: string;
  desc?: string;
  icon?: string;
  color?: string;
};

export type ScorePlusVO = {
  student_id: string;
  score_type_id: string;
  action_value: number;
  reason?: string;
};

export type Score = {
  score_type_id: string;
  score: number;
};

export type ScoreStatistics = {
  date: string;
  positive_count: number;
  negative_count: number;
  positive_total_score: number;
  negative_total_score: number;
};


