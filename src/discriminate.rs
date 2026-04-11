use crate::corpus::BigramFreq;

pub fn discriminate_language(text: &BigramFreq, bigram_freqs: &[BigramFreq]) -> String {
    let mut lowest_idx: usize = 0;
    let mut lowest_error: f32 = f32::MAX;

    for (i, bf) in bigram_freqs.iter().enumerate() {
        let error = bigram_error(text, bf);

        if error < lowest_error {
            lowest_error = error;
            lowest_idx = i;
        }
    }

    bigram_freqs[lowest_idx].name()
}

pub fn bigram_error(lhs: &BigramFreq, rhs: &BigramFreq) -> f32 {
    let mut total_err: f32 = 0.0;

    for i in 0..27 {
        for j in 0..27 {
            let lhs_freq: f32 = lhs.freq_at(i, j);
            let rhs_freq: f32 = rhs.freq_at(i, j);

            let err_square: f32 = (lhs_freq - rhs_freq).powi(2);
            total_err += err_square;
        }
    }

    total_err
}
