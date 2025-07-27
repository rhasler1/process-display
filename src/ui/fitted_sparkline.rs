use ratatui::{prelude::*, widgets::{Block, RenderDirection, Widget}};

pub struct FittedSparkline<'a> {
    pub data: &'a [u64],
    pub num_data_points: Option<u16>,
    pub max: Option<u64>,
    pub block: Option<Block<'a>>,
    pub style: Style,
    pub direction: RenderDirection,
}

impl<'a> Default for FittedSparkline<'a> {
    fn default() -> Self {
        Self {
            data: &[],
            num_data_points: None,
            max: None,
            block: None,
            style: Style::default(),
            direction: RenderDirection::LeftToRight,
        }
    }
}

// builder-style implementation
impl<'a> FittedSparkline<'a> {
    pub fn data(mut self, data: &'a [u64]) -> Self {
        self.data = data;
        self
    }

    pub fn num_data_points(mut self, num_data_points: Option<u16>) -> Self {
        self.num_data_points = num_data_points;
        self
    }

    pub fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn direction(mut self, direction: RenderDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl Widget for FittedSparkline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where Self: Sized
    {
        let spark_area = if let Some(block) = self.block {
            let inner = block.inner(area);
            block.render(area, buf);
            inner
        }
        else {
            area
        };

        let data_width = self.data.len() as u16;

        if spark_area.height == 0 || spark_area.width == 0 { return; }
        if data_width > spark_area.width { return; }                    // currently does not support truncation

        let data = self.data;
        let max = self.max.unwrap_or_else(|| data.iter().copied().max().unwrap_or(1));

        let fitted_data = fit_data_to_spark_area(data, self.num_data_points, spark_area.width);

        // render data points
        for (i, &value) in fitted_data
            .iter()
            .enumerate() {
                let x = if self.direction == RenderDirection::LeftToRight {
                    spark_area.left().saturating_add(i as u16)
                }
                else {
                    spark_area.right().saturating_sub(1).saturating_sub(i as u16)
                };
            
                let height_ratio = value as f64 / max as f64;
                let bar_height = (height_ratio * spark_area.height as f64).round() as u16;

                for dy in 0..bar_height {
                    let y = spark_area.bottom().saturating_sub(1).saturating_sub(dy);
                    buf.get_mut(x, y).set_bg(self.style.bg.unwrap_or(Color::White));
                }
            
            }

        //assert_eq!(fitted_data.len() as u16, spark_area.width);
        
        // render avg line
        let avg_line = compute_avg(data);
        for i in 0..spark_area.width {
            let x = if self.direction == RenderDirection::LeftToRight {
                spark_area.left().saturating_add(i as u16)
            }
            else {
                spark_area.right().saturating_sub(1).saturating_sub(i as u16)
            };

            let height_ratio = avg_line as f64 / max as f64;
            let bar_height = (height_ratio * spark_area.height as f64).round() as u16;

            let y = spark_area.bottom().saturating_sub(1).saturating_sub(bar_height);
            buf.get_mut(x, y).set_symbol("_").set_fg(Color::Blue);
        }
    }
}

fn compute_avg(data: &[u64]) -> u64 {
    let mut avg: u64 = 0;
    for &value in data {
        avg += value;
    }
    avg = avg / data.len() as u64;

    avg 
}

fn fit_data_to_spark_area(data: &[u64], num_data_points: Option<u16>, spark_area_width: u16) -> Vec<u64> {
    let (times, remainder) = if let Some(capacity) = num_data_points {
        ( spark_area_width / capacity, spark_area_width % capacity )
    }
    else {
        ( spark_area_width / data.len() as u16, spark_area_width % data.len() as u16 )
    };

    // fill times
    let mut result = Vec::new();
    for &value in data {
        for _ in 0..times {
            result.push(value);
        }
    }

    // fill remainder
    if let Some(last_data_point) = data.last() {
        for _ in 0..remainder {
            result.push(*last_data_point);
        }
    }

    result
}


/*
// Experimenting with custom Widget, needs some work
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()> {
        // preparing data to be drawn
        let ram_percent_usage_data: Vec<u64> = self.queue_state
            .iter()
            .rev()
            .map(|memory_item| {
                ( memory_item.used_memory_gb() / memory_item.total_memory_gb() * 100_f64 ) as u64
            })
            .collect();

        let block_style = if focused {
            self.config.theme_config.style_border_focused
        }
        else {
            self.config.theme_config.style_border_not_focused
        };

        let title: String = if let Some(memory_item) = self.queue_state.back() {
            format!(" RAM :: {:.2}/{:.2} GB :: {:.2} % ", memory_item.used_memory_gb(), memory_item.total_memory_gb(), (memory_item.used_memory_gb()/memory_item.total_memory_gb()*100_f64))
        }
        else {
            format!(" RAM ")
        };

        // see ui/fitted_sparkline.rs for implementation details
        let widget = FittedSparkline::default()
            .data(&ram_percent_usage_data)
            .num_data_points(Some(self.queue_state.capacity() as u16))
            .max(100)
            .direction(RenderDirection::RightToLeft)
            .block(Block::new().borders(Borders::ALL).title(title).style(block_style))
            .style(Style::new().on_red());

        f.render_widget(widget, area);

        Ok(())
    }
*/