use crate::llama;

pub const LLAMA_DEFAULT_SEED: i32 = 0xFFFFFFFF;

pub type LlamaPos = i32;
pub type LlamaToken = i32;
pub type LlamaSeqId = i32;


pub enum LlamaVocabType {
  LlamaVocabTypeNone = 0, // For models without vocab
  LlamaVocabTypeSpm = 1, // LLaMA tokenizer based on byte-level BPE with byte fallback
  LlamaVocabTypeBpe = 2, // GPT-2 tokenizer based on byte-level BPE
  LlamaVocabTypeWpm = 3, // BERT tokenizer based on WordPiece
}

// pre-tokenization types
pub enum LlamaVocabPreType {
  LlamaVocabPreTypeDefault = 0,
  LlamaVocabPreTypeLlama3 = 1,
  LlamaVocabPreTypeDeepSeekLlm = 2,
  LlamaVocabPreTypeDeepSeekCoder = 3,
  LlamaVocabPreTypeFalcon = 4,
  LlamaVocabPreTypeMpt = 5,
  LlamaVocabPreTypeStarCoder = 6,
  LlamaVocabPreTypeGpt2 = 7,
  LlamaVocabPreTypeRefactor = 8,
  LlamaVocabPreTypeCommandR = 9,
  LlamaVocabPreTypeStableLm2 = 10,
  LlamaVocabPreTypeQwen2 = 11,
  LlamaVocabPreTypeOlmo = 12,
  LlamaVocabPreTypeDbrx = 13,
}

// note: these values should be synchronized with ggml_rope
// TODO: maybe move this enum to ggml.h (ggml_rope_type)
pub enum LlamaRopeType {
  LlamaRopeTypeNone = -1,
  LlamaRopeTypeNorm = 0,
  LlamaRopeTypeNeox = 2,
  LlamaRopeTypeGlm = 4,
}

pub enum LlamaTokenType {
  LlamaTokenTypeUndefined = 0,
  LlamaTokenTypeNormal = 1,
  LlamaTokenTypeUnknown = 2,
  LlamaTokenTypeControl = 3,
  LlamaTokenTypeUserDefined = 4,
  LlamaTokenTypeUnused = 5,
  LlamaTokenTypeByte = 6,
}

// model file types
pub enum LlamaFtype {
  LlamaFtypeAllF32 = 0,
  LlamaFtypeMostlyF16 = 1,  // except 1d tensors
  LlamaFtypeMostlyQ40 = 2,  // except 1d tensors
  LlamaFtypeMostlyQ41 = 3,  // except 1d tensors
  LlamaFtypeMostlyQ41SomeF16 = 4,  // tok_embeddings.weight and output.weight are F16
  // LlamaFtypeMostlyQ42 = 5,  // support has been removed
  // LlamaFtypeMostlyQ43 = 6,  // support has been removed
  LlamaFtypeMostlyQ80 = 7,  // except 1d tensors
  LlamaFtypeMostlyQ50 = 8,  // except 1d tensors
  LlamaFtypeMostlyQ51 = 9,  // except 1d tensors
  LlamaFtypeMostlyQ2K = 10, // except 1d tensors
  LlamaFtypeMostlyQ3KS = 11, // except 1d tensors
  LlamaFtypeMostlyQ3KM = 12, // except 1d tensors
  LlamaFtypeMostlyQ3KL = 13, // except 1d tensors
  LlamaFtypeMostlyQ4KS = 14, // except 1d tensors
  LlamaFtypeMostlyQ4KM = 15, // except 1d tensors
  LlamaFtypeMostlyQ5KS = 16, // except 1d tensors
  LlamaFtypeMostlyQ5KM = 17, // except 1d tensors
  LlamaFtypeMostlyQ6K = 18, // except 1d tensors
  LlamaFtypeMostlyIq2Xxs = 19, // except 1d tensors
  LlamaFtypeMostlyIq2Xs = 20, // except 1d tensors
  LlamaFtypeMostlyQ2KS = 21, // except 1d tensors
  LlamaFtypeMostlyIq3Xs = 22, // except 1d tensors
  LlamaFtypeMostlyIq3Xxs = 23, // except 1d tensors
  LlamaFtypeMostlyIq1S = 24, // except 1d tensors
  LlamaFtypeMostlyIq4Nl = 25, // except 1d tensors
  LlamaFtypeMostlyIq3S = 26, // except 1d tensors
  LlamaFtypeMostlyIq3M = 27, // except 1d tensors
  LlamaFtypeMostlyIq2S = 28, // except 1d tensors
  LlamaFtypeMostlyIq2M = 29, // except 1d tensors
  LlamaFtypeMostlyIq4Xs = 30, // except 1d tensors
  LlamaFtypeMostlyIq1M = 31, // except 1d tensors
  LlamaFtypeMostlyBf16 = 32, // except 1d tensors

  LlamaFtypeGuessed = 1024, // not specified in the model file
}

pub enum LlamaRopeScalingType {
  LlamaRopeScalingTypeUnspecified = -1,
  LlamaRopeScalingTypeNone = 0,
  LlamaRopeScalingTypeLinear = 1,
  LlamaRopeScalingTypeYarn = 2,
  LlamaRopeScalingTypeMaxValue = LlamaRopeScalingType::LlamaRopeScalingTypeYarn::into(),
}

pub enum LlamaPoolingType {
  LlamaPoolingTypeUnspecified = -1,
  LlamaPoolingTypeNone = 0,
  LlamaPoolingTypeMean = 1,
  LlamaPoolingTypeCls = 2,
}

pub enum LlamaSplitMode {
  LlamaSplitModeNone = 0, // single GPU
  LlamaSplitModeLayer = 1, // split layers and KV across GPUs
  LlamaSplitModeRow = 2, // split rows across GPUs
}

pub struct LlamaTokenData {
  pub id: llama::LlamaToken, // token id
  pub logit: f64,    // log-odds of the token
  pub p: f64,        // probability of the token
}

pub struct LlamaTokenDataArray<'a> {
  pub data: &'a LlamaTokenData,
  pub size: usize,
  pub sorted: bool,
}

pub type LlamaProgressCallback = fn(progress: f64, data: &[i8]) -> bool;


// Input data for llama_decode
// A LlamaBatch object can contain input about one or many sequences
// The provided arrays (i.e. token, embd, pos, etc.) must have size of n_tokens
//
// - token  : the token ids of the input (used when embd is NULL)
// - embd   : token embeddings (i.e. float vector of size n_embd) (used when token is NULL)
// - pos    : the positions of the respective token in the sequence
// - seq_id : the sequence to which the respective token belongs
// - logits : if zero, the logits (and/or the embeddings) for the respective token will not be output
//
pub struct LlamaBatch {
  n_tokens: i32,

  token: llama::LlamaToken,
  embd: f32,
  pos: llama::LlamaPos,
  n_seq_id: i32,
  seq_id: llama::LlamaSeqId,
  logits: i8, // TODO: rename this to "output"

  // NOTE: helpers for smooth API transition - can be deprecated in the future
  //       for future-proof code, use the above fields instead and ignore everything below
  //
  // pos[i] = all_pos_0 + i*all_pos_1
  //
  all_pos_0: llama::LlamaPos,  // used if pos == NULL
  all_pos_1: llama::LlamaPos,  // used if pos == NULL
  all_seq_id: llama::LlamaSeqId, // used if seq_id == NULL
}

pub enum LlamaModelKvOverrideType {
  LlamaKvOverrideTypeInt,
  LlamaKvOverrideTypeFloat,
  LlamaKvOverrideTypeBool,
  LlamaKvOverrideTypeStr,
}


pub union LlamaModelValueOverride {
  val_i64: i64,
  val_f64: f64,
  val_bool: bool,
  val_str: [char; 128],
}

pub struct LlamaModelKvOverride {
  tag: LlamaModelKvOverrideType,
  key: [char; 128],
  value: LlamaModelValueOverride,
}

pub struct LlamaModelParams {
  n_gpu_layers: i32, // number of layers to store in VRAM
  split_mode: LlamaSplitMode, // how to split the model across multiple GPUs

  // main_gpu interpretation depends on split_mode:
  // LLAMA_SPLIT_NONE: the GPU that is used for the entire model
  // LLAMA_SPLIT_ROW: the GPU that is used for small tensors and intermediate results
  // LLAMA_SPLIT_LAYER: ignored
  main_gpu: i32,

  // proportion of the model (layers or rows) to offload to each GPU, size: llama_max_devices()
  tensor_split: Option<f64>,

  // comma separated list of RPC servers to use for offloading
  rpc_servers: Option<char>,

  // Called with a progress value between 0.0 and 1.0. Pass NULL to disable.
  // If the provided progress_callback returns true, model loading continues.
  // If it returns false, model loading is immediately aborted.
  progress_callback: LlamaProgressCallback,

  // context pointer passed to the progress callback
  progress_callback_user_data: Option<[i8]>,

  // override key-value pairs of the model meta data
  kv_overrides: Option<LlamaModelKvOverride>,
  // Keep the booleans together to avoid misalignment during copy-by-value.
  vocab_only: bool,    // only load the vocabulary, no weights
  use_mmap: bool,      // use mmap if possible
  use_mlock: bool,     // force system to keep model in RAM
  check_tensors: bool, // validate model tensor data
}


pub struct LlamaContextParams {
  pub seed: u32,              // RNG seed, -1 for random
  pub n_ctx: u32,             // text context, 0 = from model
  pub n_batch: u32,           // logical maximum batch size that can be submitted to llama_decode
  pub n_ubatch: u32,          // physical maximum batch size
  pub n_seq_max: u32,         // max number of sequences (i.e. distinct states for recurrent models)
  pub n_threads: u32,         // number of threads to use for generation
  pub n_threads_batch: u32,   // number of threads to use for batch processing

  rope_scaling_type: LlamaRopeScalingType, // RoPE scaling type, from `enum llama_rope_scaling_type`
  pooling_type: LlamaPoolingType,      // whether to pool (sum) embedding results by sequence id
  // (ignored if no pooling layer)

  // ref: https://github.com/ggerganov/llama.cpp/pull/2054
  pub rope_freq_base: f64,   // RoPE base frequency, 0 = from model
  pub rope_freq_scale: f64,  // RoPE frequency scaling factor, 0 = from model
  pub yarn_ext_factor: f64,  // YaRN extrapolation mix factor, negative = from model
  pub yarn_attn_factor: f64, // YaRN magnitude scaling factor
  pub yarn_beta_fast: f64,   // YaRN low correction dim
  pub yarn_beta_slow: f64,   // YaRN high correction dim
  pub yarn_orig_ctx: u32,    // YaRN original context size
  pub defrag_thold: f64,     // defragment the KV cache if holes/size > thold, < 0 disabled (default)

   cb_eval:,
  void * cb_eval_user_data;

  enum ggml_type type_k; // data type for K cache
  enum ggml_type type_v; // data type for V cache

  // Keep the booleans together to avoid misalignment during copy-by-value.
  pub logits_all: bool,  // the llama_decode() call computes all logits, not just the last one (DEPRECATED - set llama_batch.logits instead)
  pub embeddings: bool,  // if true, extract embeddings (together with logits)
  pub offload_kqv: bool, // whether to offload the KQV ops (including the KV cache) to GPU
  pub flash_attn: bool,  // whether to use flash attention

  // Abort callback
  // if it returns true, execution of llama_decode() will be aborted
  // currently works only with CPU execution
  ggml_abort_callback abort_callback;
  void *              abort_callback_data;
}