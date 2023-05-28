from bottle import route, request, run, abort
import pickle
from sklearn import svm
from sklearn.feature_extraction.text import CountVectorizer, TfidfTransformer
import re


class HateSpeechPredictor:
    lin_clf: svm.LinearSVC
    count_vect: CountVectorizer
    transformer: TfidfTransformer
    
    def __init__(self):
        model_file = open("model", "rb")
        count_vect_file = open("count_vect", "rb")
        transformer_file = open("transformer", "rb")
        
        self.lin_clf = pickle.load(model_file)
        self.count_vect = pickle.load(count_vect_file)
        self.transformer = pickle.load(transformer_file)
        
        model_file.close()
        count_vect_file.close()
        transformer_file.close()

    def __process_tweet(sellf, tweet):
        return " ".join(re.sub("(@[A-Za-z0-9]+)|([^0-9A-Za-z \t])", " ",tweet.lower()).split())

    def predict(self, text):
        text = self.__process_tweet(text)
        text = [text]
        
        text_counts = self.count_vect.transform(text)
        text_tfidf = self.transformer.transform(text_counts)

        return self.lin_clf.predict(text_tfidf)[0]


hate_speech_predictor = None

@route('/api/hate-speech', method='POST')
def detect_hate_speech():
    text = request.body.read().decode('utf-8')
    if hate_speech_predictor.predict(text) == 1:
        abort(400, "Hate speech detected")
    else:
        return "ok"


if __name__ == '__main__':
    hate_speech_predictor = HateSpeechPredictor()
    run(host='0.0.0.0', port=8080)
    